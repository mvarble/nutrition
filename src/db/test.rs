use anyhow::Result;
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool},
    RunQueryDsl,
};

use crate::{
    db::{api::Food, DB},
    env,
};

fn establish_test_db() -> Result<DB> {
    let env = env::get()?;
    let manager = ConnectionManager::<PgConnection>::new(format!(
        "{}/{}",
        env.database_url, env.database_name
    ));
    let builder = Pool::builder().max_size(1);
    let pool = builder.build(manager)?;
    let db = DB { pool };
    Ok(db)
}

#[test]
fn get_foods() {
    // establish a database and a connection
    let db = establish_test_db().unwrap();
    let conn = db.connect().unwrap();

    // run a query on some arbitrary fdc_id vector
    let fdc_ids = [1438282, 1438339, 1438425, 1441890, 1441964];
    let foods = DB::get_foods(&conn, &fdc_ids).unwrap();
    println!("{:#?}", foods);
    // TODO: make test
}
