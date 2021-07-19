//! This module holds Postgres DTO structs

use chrono::{DateTime, Utc};
use uom::si::{mass::gram, mass_density::kilogram_per_cubic_meter};

use crate::{
    nutritionix::NutritionixFood,
    schema::{foods, meals, servings},
    si::MeasurementType,
};

/// This struct corresponds with the `foods` table of our database.
#[derive(Identifiable, Queryable, PartialEq, Debug)]
pub struct Food {
    pub id: i32,
    pub name: String,
    pub mass: f32,
    pub nutrition: Vec<f32>,
    pub g2l_density: Option<f32>,
    pub img: Option<String>,
    pub brand: Option<String>,
    pub upc: Option<String>,
}

#[derive(Insertable, PartialEq, Debug)]
#[table_name = "foods"]
pub struct FoodForm {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub mass: Option<f32>,
    pub nutrition: Option<Vec<f32>>,
    pub g2l_density: Option<f32>,
    pub img: Option<String>,
    pub brand: Option<String>,
    pub upc: Option<String>,
}

/// This struct corresponds with the `servings` table of our database.
#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Food)]
#[table_name = "servings"]
pub struct Serving {
    pub id: i32,
    pub food_id: i32,
    pub name: String,
    pub mass: f32,
}

#[derive(Insertable, PartialEq, Debug)]
#[table_name = "servings"]
pub struct ServingForm {
    pub id: Option<i32>,
    pub food_id: Option<i32>,
    pub name: Option<String>,
    pub mass: Option<f32>,
}

impl ServingForm {
    pub fn update_fid(&mut self, fid: i32) {
        self.food_id = Some(fid);
    }
}

/// This struct corresponds with the `meals` table of our database.
#[derive(Identifiable, Queryable, Serialize, Deserialize, PartialEq, Debug)]
pub struct Meal {
    pub id: i32,
    pub name: Option<String>,
    pub time: DateTime<Utc>,
    pub servings: f32,
    pub servings_consumed: f32,
    pub food_ids: Vec<i32>,
    pub food_masses: Vec<f32>,
}

#[derive(Insertable, PartialEq, Debug)]
#[table_name = "meals"]
pub struct MealForm {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub time: Option<DateTime<Utc>>,
    pub servings: Option<f32>,
    pub servings_consumed: Option<f32>,
    pub food_ids: Option<Vec<i32>>,
    pub food_masses: Option<Vec<f32>>,
}

/// Map Nutritionix payloads to forms for the Postgres insertions.
pub fn parsenix(nixfood: NutritionixFood) -> (FoodForm, Vec<ServingForm>) {
    // calculate the mass and nutrition from the food
    let mass = nixfood.serving_weight_grams / nixfood.serving_qty;
    let nutrition = nixfood
        .full_nutrients
        .iter()
        .flat_map(|n| vec![n.attr_id as f32, n.value].into_iter())
        .collect::<Vec<_>>();

    // fold over the measurements to get nominals and a density
    let (g2l_density, servings) = nixfood.alt_measures.unwrap_or_default().into_iter().fold(
        (None, Vec::<ServingForm>::new()),
        |(g2l_density, mut servings), measure| {
            let mtype = MeasurementType::from(measure);
            match mtype {
                MeasurementType::Masslike => (g2l_density, servings),
                MeasurementType::MassDensity(density) => {
                    let proposal = density.get::<kilogram_per_cubic_meter>();
                    (g2l_density.or(Some(proposal)), servings)
                }
                MeasurementType::Nominal(name, mass) => {
                    let serving_form = ServingForm {
                        id: None,
                        food_id: None,
                        name: Some(name),
                        mass: Some(mass.get::<gram>()),
                    };
                    servings.push(serving_form);
                    (g2l_density, servings)
                }
            }
        },
    );

    // create the form and measurements
    (
        FoodForm {
            id: None,
            name: Some(nixfood.food_name),
            mass: Some(mass),
            nutrition: Some(nutrition),
            g2l_density,
            img: None,
            brand: nixfood.brand_name,
            upc: nixfood.upc,
        },
        servings,
    )
}
