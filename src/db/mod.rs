//! Expose the `DB` struct, which wraps a Postgres pool and implements specific queries.

use anyhow::{Error, Result};
use diesel::{
    pg::{Pg, PgConnection},
    prelude::*,
    r2d2::{self, ConnectionManager, PooledConnection},
    RunQueryDsl,
};
use itertools::Itertools;

use crate::nutritionix::NutritionixFood;
use crate::schema::{foods, servings};

pub mod api;
pub mod pg;
#[cfg(test)]
mod test;

/// This struct wraps a Postgres pool and implements specific queries for our database.
#[derive(Clone)]
pub struct DB {
    pool: r2d2::Pool<ConnectionManager<PgConnection>>,
}

impl DB {
    /// Create a new Postgres pool.
    pub fn new<S: Into<String>>(path: S) -> Result<DB> {
        let manager = ConnectionManager::<PgConnection>::new(path);
        let builder = diesel::r2d2::Pool::builder();
        let pool = builder.build(manager)?;
        let db = DB { pool };
        Ok(db)
    }

    /// Establish a connection to a thread of the pool.
    pub fn connect(&self) -> Result<PooledConnection<ConnectionManager<PgConnection>>> {
        self.pool.get().map_err(Error::new)
    }

    /// Store Nutritionix payloads in the database. There are no upserts; if a food already exists,
    /// it and the servings are immediately ignored.
    pub fn add_foods<C: Connection<Backend = Pg>>(
        conn: &C,
        nixfoods: Vec<NutritionixFood>,
    ) -> Result<Vec<api::Food>> {
        // parse the Nutritionix payloads for database queries
        let (vec_foods, mut vec_servings): (Vec<pg::FoodForm>, Vec<Vec<pg::ServingForm>>) =
            nixfoods.into_iter().map(pg::parsenix).unzip();

        // insert the food forms, returning those not already in database
        let inserted_foods: Vec<pg::Food> = diesel::insert_into(foods::table)
            .values(&vec_foods)
            .on_conflict_do_nothing()
            .returning(foods::all_columns)
            .get_results(conn)
            .map_err(Error::from)?;

        // mutate the serving forms to now have id's
        inserted_foods.iter().for_each(|food| {
            let opt_servings = vec_servings
                .iter_mut()
                .zip(&vec_foods)
                .find(|(servings, food_form)| {
                    food_form.name.as_ref() == Some(&food.name)
                        && food_form.brand == food.brand
                        && food_form.upc == food.upc
                        && !servings.is_empty()
                })
                .map(|(servings, _)| servings);
            if let Some(servings) = opt_servings {
                servings
                    .iter_mut()
                    .for_each(|serving| serving.update_fid(food.id));
            }
        });

        // collect the serving forms to one flattened vector
        let vec_servings: Vec<pg::ServingForm> = vec_servings
            .into_iter()
            .filter(|serving| !serving.is_empty() && serving.get(0).unwrap().food_id.is_some())
            .flatten()
            .collect();

        // insert into database
        let inserted_servings: Vec<pg::Serving> = diesel::insert_into(servings::table)
            .values(&vec_servings)
            .on_conflict_do_nothing()
            .returning(servings::all_columns)
            .get_results(conn)
            .map_err(Error::from)?;

        // recollect the inserted foods and servings into a Vec<api::Food>
        let mut inserted_servings_grouped = inserted_servings
            .into_iter()
            .into_group_map_by(|serving| serving.food_id);
        let foods = inserted_foods
            .into_iter()
            .map(|food| {
                let opt_servings = inserted_servings_grouped.remove_entry(&food.id);
                if let Some((_, servings)) = opt_servings {
                    api::parsedb(food, servings)
                } else {
                    api::parsedb(food, Vec::new())
                }
            })
            .collect::<Vec<api::Food>>();
        Ok(foods)
    }

    /// for each food, grab the servings and map to the api object
    fn append_servings<C: Connection<Backend = Pg>>(
        conn: &C,
        db_foods: Vec<pg::Food>,
    ) -> Result<Vec<api::Food>> {
        let len = db_foods.len();
        db_foods
            .into_iter()
            .fold(
                Ok(Vec::with_capacity(len)),
                |api_foods_res, food| match api_foods_res {
                    Ok(mut api_foods) => {
                        let food_res = pg::Serving::belonging_to(&food)
                            .load::<pg::Serving>(conn)
                            .map(|food_servings| api::parsedb(food, food_servings))
                            .map_err(Error::from);
                        match food_res {
                            Ok(food) => {
                                api_foods.push(food);
                                Ok(api_foods)
                            }
                            Err(e) => Err(e),
                        }
                    }
                    Err(e) => Err(e),
                },
            )
    }

    /// Request foods from database by ids
    pub fn get_foods_by_ids<C: Connection<Backend = Pg>>(
        conn: &C,
        fids: &[i32],
    ) -> Result<Vec<api::Food>> {
        use crate::schema::foods::dsl::*;
        let db_foods = foods.filter(id.eq_any(fids)).load(conn)?;
        DB::append_servings(conn, db_foods)
    }

    /// Request foods from database by upcs
    pub fn get_foods_by_upcs<C: Connection<Backend = Pg>>(
        conn: &C,
        upcs: &[String],
    ) -> Result<Vec<api::Food>> {
        use crate::schema::foods::dsl::*;
        let db_foods = foods.filter(upc.eq_any(upcs)).load(conn)?;
        DB::append_servings(conn, db_foods)
    }

    /// Request foods by name
    pub fn get_foods_by_name<C: Connection<Backend = Pg>>(
        conn: &C,
        likename: &str,
    ) -> Result<Vec<api::Food>> {
        use crate::schema::foods::dsl::*;
        let db_foods = foods
            .filter(name.ilike(format!("{}%", likename)))
            .load(conn)?;
        DB::append_servings(conn, db_foods)
    }
}
