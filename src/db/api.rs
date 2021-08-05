//! This module holds the objects which we use for our API

use anyhow::{anyhow, Error, Result};
use chrono::NaiveDate;

use super::orm;

#[derive(Debug, PartialEq)]
pub struct Food {
    pub fdc_id: i32,
    pub description: Option<String>,
    pub publication_date: NaiveDate,
    pub food_nutrients: Vec<FoodNutrient>,
    pub data_type: String,
    pub food_class: String,
    pub modified_date: Option<NaiveDate>,
    pub available_date: Option<NaiveDate>,
    pub brand_owner: Option<String>,
    pub brand_name: Option<String>,
    pub subbrand_name: Option<String>,
    pub data_source: String,
    pub branded_food_category: Option<String>,
    pub gtin_upc: Option<String>,
    pub household_serving_fulltext: Option<String>,
    pub ingredients: Option<String>,
    pub market_country: String,
    pub serving_size: Option<f32>,
    pub serving_size_unit: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct FoodNutrient {
    pub nutrient: Nutrient,
    pub food_nutrient_derivation: Option<FoodNutrientDerivation>,
    pub id: i32,
    pub amount: f32,
    pub data_points: i32,
}

#[derive(Debug, PartialEq)]
pub struct Nutrient {
    pub id: i32,
    pub number: String,
    pub name: String,
    pub rank: i32,
    pub unit_name: String,
}

#[derive(Debug, PartialEq)]
pub struct FoodNutrientDerivation {
    pub id: i32,
    pub code: String,
    pub description: String,
    pub food_nutrient_source: Option<orm::FoodNutrientSource>,
}

impl From<(orm::FoodNutrientDerivation, Option<orm::FoodNutrientSource>)>
    for FoodNutrientDerivation
{
    fn from(tuple: (orm::FoodNutrientDerivation, Option<orm::FoodNutrientSource>)) -> Self {
        let (fnd, food_nutrient_source) = tuple;
        Self {
            id: fnd.id,
            code: fnd.code,
            description: fnd.description,
            food_nutrient_source,
        }
    }
}

use std::convert::TryFrom;

impl TryFrom<(orm::Food, Option<orm::BrandedFood>)> for Food {
    type Error = Error;
    fn try_from(tuple: (orm::Food, Option<orm::BrandedFood>)) -> Result<Self> {
        match tuple {
            (food, Some(branded_food)) => Ok(Self {
                fdc_id: food.fdc_id,
                description: food.description,
                publication_date: food.publication_date,
                food_nutrients: vec![], // TODO: fill this in
                data_type: "Branded".into(),
                food_class: "Branded".into(),
                modified_date: branded_food.modified_date,
                available_date: branded_food.available_date,
                brand_owner: branded_food.brand_owner,
                brand_name: branded_food.brand_name,
                subbrand_name: branded_food.subbrand_name,
                data_source: branded_food.data_source,
                branded_food_category: branded_food.branded_food_category,
                gtin_upc: branded_food.gtin_upc,
                household_serving_fulltext: branded_food.household_serving_fulltext,
                ingredients: branded_food.ingredients,
                market_country: branded_food.market_country,
                serving_size: branded_food.serving_size,
                serving_size_unit: branded_food.serving_size_unit,
            }),
            // currently reserving this to be the impossible (food, None, ..., None) case
            (food, None) => Err(anyhow!(
                "Unexpected: each food should match some `data_type`."
            )),
        }
    }
}
