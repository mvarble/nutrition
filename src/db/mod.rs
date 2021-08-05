//! Expose the `DB` struct, which wraps a Postgres pool and implements specific queries

use anyhow::{Error, Result};
use diesel::{
    pg::{Pg, PgConnection},
    prelude::*,
    r2d2::{self, ConnectionManager, PooledConnection},
    RunQueryDsl,
};
use std::convert::TryFrom;

use crate::schema::{
    branded_food, food, food_nutrient, food_nutrient_derivation, food_nutrient_source, nutrient,
};

mod api;
mod orm;
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

    /// Get foods from fdc_ids
    pub fn get_foods<C: Connection<Backend = Pg>>(conn: &C, ids: &[i32]) -> Result<Vec<api::Food>> {
        // data_types we want:
        // [ ] agricultural_acquisition
        // [ ] branded_food
        // [X] experimental_food            (don't care)
        // [ ] foundation_food
        // [ ] market_acquistion
        // [ ] sample_food
        // [ ] sr_legacy_food
        // [ ] sub_sample_food
        // [ ] survey_fndds_food
        let orm_foods = food::table
            .left_join(branded_food::table)
            .filter(food::fdc_id.eq_any(ids))
            .load::<(orm::Food, Option<orm::BrandedFood>)>(conn)?;

        let orm_base_foods: Vec<&orm::Food> = orm_foods.iter().map(|t| &t.0).collect();
        let nutrients = orm::FoodNutrient::belonging_to(orm_base_foods).load(conn)?;

        orm_foods
            .into_iter()
            .map(|orm_food| {
                let food_res: Result<api::Food> = api::Food::try_from(orm_food);
                food_res
            })
            .collect::<Result<Vec<api::Food>>>()
    }
}

use diesel::associations::{BelongsTo, HasTable, Identifiable};
impl<'a, Parent, Child> BelongingToDsl<&Vec<&'a Parent>> for Child
where
    &'a Parent: Identifiable,
    Child: HasTable + BelongsTo<Parent>,
    Vec<Id<&'a Parent>>: AsInExpression<<Child::ForeignKeyColumn as Expression>::SqlType>,
    <Child as HasTable>::Table: FilterDsl<EqAny<Child::ForeignKeyColumn, Vec<Id<&'a Parent>>>>,
    Child::ForeignKeyColumn: ExpressionMethods,
{
    type Output = Filter<Child::Table, EqAny<Child::ForeignKeyColumn, Vec<Id<&'a Parent>>>>;

    fn belonging_to(parents: &'a [Parent]) -> Self::Output {
        let ids = parents.iter().map(Identifiable::id).collect::<Vec<_>>();
        FilterDsl::filter(Child::table(), Child::foreign_key_column().eq_any(ids))
    }
}
