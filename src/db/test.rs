use anyhow::Result;
use diesel::{pg::PgConnection, r2d2::ConnectionManager, RunQueryDsl};

use crate::{
    db::{api, pg, DB},
    embedded_migrations, env,
    schema::{foods, meals, servings},
    si::test::get_nutritionix_payload,
};

#[test]
fn parsedb() {
    let dbfood = pg::Food {
        id: 1,
        name: "hi".to_string(),
        mass: 6.9,
        nutrition: Vec::new(),
        g2l_density: Some(10.3),
        img: None,
        brand: None,
        upc: None,
    };
    let dbservings = vec![
        pg::Serving {
            id: 0,
            food_id: 1,
            name: "slice".to_string(),
            mass: 50.0,
        },
        pg::Serving {
            id: 0,
            food_id: 1,
            name: "cube".to_string(),
            mass: 1.0,
        },
    ];
    let food = api::parsedb(dbfood, dbservings);
    assert_eq!(food.name, "hi");
    assert_eq!(food.mass, 6.9);
    assert!(!food.measurements.is_empty());
    assert_eq!(
        food.measurements.get(0),
        Some(api::Measurement {
            measurement_type: api::MeasurementType::Mass,
            name: "milligram".to_string(),
            mass: 0.001,
        })
        .as_ref(),
    );
    assert_eq!(
        food.measurements.get(5),
        Some(api::Measurement {
            measurement_type: api::MeasurementType::Volume,
            name: "milliliter".to_string(),
            mass: 0.0103,
        })
        .as_ref(),
    );
    assert_eq!(
        food.measurements.get(16),
        Some(api::Measurement {
            measurement_type: api::MeasurementType::Nominal,
            name: "slice".to_string(),
            mass: 50.0,
        })
        .as_ref(),
    );
}

fn establish_test_db() -> Result<DB> {
    let env = env::get()?;
    let manager = ConnectionManager::<PgConnection>::new(format!(
        "{}/{}_testing",
        env.database_url, env.database_name
    ));
    let builder = diesel::r2d2::Pool::builder().max_size(1);
    let pool = builder.build(manager)?;
    let db = DB { pool };
    let conn = db.connect()?;
    embedded_migrations::run(&conn)?;
    diesel::delete(servings::table).execute(&conn)?;
    diesel::delete(meals::table).execute(&conn)?;
    diesel::delete(foods::table).execute(&conn)?;
    Ok(db)
}

fn assert_payload(food: &api::Food) {
    match &food.name[..] {
        "egg" => {
            assert_eq!(food.nutrition.get("Energy"), Some(&71.5));
            assert_eq!(food.nutrition.get("Cholesterol"), Some(&186.0));
            assert!(food.measurements.iter().any(|meas| {
                meas.measurement_type == api::MeasurementType::Nominal
                    && meas.name == "small".to_string()
                    && meas.mass == 38.0
            }));
        }
        "mayo" => {
            assert_eq!(food.nutrition.get("Energy"), Some(&215.441));
            assert_eq!(food.nutrition.get("Cholesterol"), Some(&13.3067));
            assert!(food.measurements.iter().any(|meas| {
                meas.measurement_type == api::MeasurementType::Nominal
                    && meas.name == "packet".to_string()
                    && meas.mass == 10.0
            }));
        }
        "jam" => {
            assert_eq!(food.nutrition.get("Energy"), Some(&38.92));
            assert_eq!(food.nutrition.get("Cholesterol"), Some(&0.0));
            assert!(food.measurements.iter().any(|meas| {
                meas.measurement_type == api::MeasurementType::Nominal
                    && meas.name == "packet (0.5 oz)".to_string()
                    && meas.mass == 14.0
            }));
        }
        "whole wheat toast" => {
            assert_eq!(food.nutrition.get("Energy"), Some(&153.0));
            assert_eq!(food.nutrition.get("Cholesterol"), Some(&0.0));
            assert!(food.measurements.iter().any(|meas| {
                meas.measurement_type == api::MeasurementType::Nominal
                    && meas.name == "slice".to_string()
                    && meas.mass == 25.0
            }));
        }
        "bacon" => {
            assert_eq!(food.nutrition.get("Energy"), Some(&161.46));
            assert_eq!(food.nutrition.get("Cholesterol"), Some(&34.155));
            assert!(food.measurements.iter().any(|meas| {
                meas.measurement_type == api::MeasurementType::Nominal
                    && meas.name == "side".to_string()
                    && meas.mass == 34.5
            }));
        }
        "milk" => {
            assert_eq!(food.nutrition.get("Energy"), Some(&528.0893));
            assert_eq!(food.nutrition.get("Cholesterol"), Some(&82.8375));
            assert!(food
                .measurements
                .iter()
                .all(|meas| { meas.measurement_type != api::MeasurementType::Nominal }));
        }
        "orange juice" => {
            assert_eq!(food.nutrition.get("Energy"), Some(&4999.68));
            assert_eq!(food.nutrition.get("Cholesterol"), Some(&0.0));
            assert!(food.measurements.iter().any(|meas| {
                meas.measurement_type == api::MeasurementType::Nominal
                    && meas.name == "fruit yields".to_string()
                    && meas.mass == 86.0
            }));
        }
        "spinach" => {
            assert_eq!(food.nutrition.get("Energy"), Some(&58.68));
            assert_eq!(food.nutrition.get("Cholesterol"), Some(&0.0));
            assert!(food
                .measurements
                .iter()
                .all(|meas| { meas.measurement_type != api::MeasurementType::Nominal }));
        }
        "Chili  Paste, Chili Paste, Ground Fresh, Sambal Oelek" => {
            assert!(food.measurements.is_empty());
            assert_eq!(food.brand, Some("Huy Fong Foods".into()));
        }
        _ => panic!("{} was not an expected food name!", food.name),
    }
}

#[test]
fn add_foods() {
    // create a test database and establish a connection
    let db = establish_test_db().unwrap();
    let conn = db.connect().unwrap();

    // test if database has objects
    let db_foods = foods::table.load::<pg::Food>(&conn).unwrap();
    assert!(db_foods.is_empty());

    // add the objects
    let nixfoods = get_nutritionix_payload().unwrap();
    let api_foods = DB::add_foods(&conn, nixfoods).unwrap();

    // test if database has objects
    let db_foods = foods::table.load::<pg::Food>(&conn).unwrap();
    assert_eq!(db_foods.len(), 9);

    // test if REST API object is correct
    api_foods.iter().for_each(|food| assert_payload(&food));
}

#[test]
fn get_foods_by_ids() {
    // create a test database and establish a connection
    let db = establish_test_db().unwrap();
    let conn = db.connect().unwrap();
    let nixfoods = get_nutritionix_payload().unwrap();
    DB::add_foods(&conn, nixfoods).unwrap();

    // grab the parsed tables
    let db_foods = foods::table.load::<pg::Food>(&conn).unwrap();
    let api_foods = DB::append_servings(&conn, db_foods).unwrap();
    let api_foods = api_foods
        .into_iter()
        .filter(|food| food.id % 2 == 0)
        .collect::<Vec<_>>();

    // check if grabbing by (the even) ids works
    let ids = api_foods.iter().map(|food| food.id).collect::<Vec<i32>>();
    let api_foods_ = DB::get_foods_by_ids(&conn, &ids).unwrap();
    api_foods
        .into_iter()
        .zip(api_foods_)
        .for_each(|(food, food_)| {
            assert_eq!(food.id, food_.id);
            assert_eq!(food.name, food_.name);
            assert_eq!(food.brand, food_.brand);
            assert_eq!(food.upc, food_.upc);
            assert_eq!(food.nutrition["Energy"], food_.nutrition["Energy"]);
        });
}

#[test]
fn get_foods_by_upcs() {
    // create a test database and establish a connection
    let db = establish_test_db().unwrap();
    let conn = db.connect().unwrap();
    let nixfoods = get_nutritionix_payload().unwrap();
    DB::add_foods(&conn, nixfoods).unwrap();

    // grab the parsed tables
    let db_foods = foods::table.load::<pg::Food>(&conn).unwrap();
    let api_foods = DB::append_servings(&conn, db_foods).unwrap();

    // check if grabbing by upc works
    let mut db_foods = DB::get_foods_by_upcs(&conn, &["024463061071".into()]).unwrap();
    assert_eq!(db_foods.len(), 1);
    let chili_paste = db_foods.pop().unwrap();
    assert_eq!(
        chili_paste.name,
        "Chili  Paste, Chili Paste, Ground Fresh, Sambal Oelek".to_string()
    );
    assert!(api_foods.into_iter().any(|food| food == chili_paste));
}

#[test]
fn get_foods_by_name() {
    // create a test database and establish a connection
    let db = establish_test_db().unwrap();
    let conn = db.connect().unwrap();
    let nixfoods = get_nutritionix_payload().unwrap();
    DB::add_foods(&conn, nixfoods).unwrap();

    // grab the parsed tables
    let db_foods = foods::table.load::<pg::Food>(&conn).unwrap();
    let api_foods = DB::append_servings(&conn, db_foods).unwrap();

    // check if grabbing by `mil` works
    let mut db_foods = DB::get_foods_by_name(&conn, &"mil".to_string()).unwrap();
    assert_eq!(db_foods.len(), 1);
    let milk = db_foods.pop().unwrap();
    assert_eq!(milk.name, "milk".to_string());
    assert!(api_foods.into_iter().any(|food| food == milk));
}
