//! This module holds the objects which interface with the database

use chrono::NaiveDate;

use crate::schema::{
    branded_food, food, food_nutrient, food_nutrient_derivation, food_nutrient_source, nutrient,
};

#[derive(Identifiable, Queryable, PartialEq, Debug)]
#[primary_key(fdc_id)]
#[table_name = "food"]
pub struct Food {
    pub fdc_id: i32,
    pub data_type: String,
    pub description: Option<String>,
    pub food_category_id: Option<i32>,
    pub publication_date: NaiveDate,
}

#[derive(Identifiable, Queryable, PartialEq, Debug)]
#[primary_key(fdc_id)]
#[table_name = "branded_food"]
pub struct BrandedFood {
    pub fdc_id: i32,
    pub brand_owner: Option<String>,
    pub brand_name: Option<String>,
    pub subbrand_name: Option<String>,
    pub gtin_upc: Option<String>,
    pub ingredients: Option<String>,
    pub not_a_significant_source_of: Option<String>,
    pub serving_size: Option<f32>,
    pub serving_size_unit: Option<String>,
    pub household_serving_fulltext: Option<String>,
    pub branded_food_category: Option<String>,
    pub data_source: String,
    pub modified_date: Option<NaiveDate>,
    pub available_date: Option<NaiveDate>,
    pub market_country: String,
    pub discontinued_date: Option<NaiveDate>,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Food, foreign_key = "fdc_id")]
#[table_name = "food_nutrient"]
pub struct FoodNutrient {
    pub id: i32,
    pub fdc_id: i32,
    pub nutrient_id: i32,
    pub amount: f32,
    pub data_points: Option<i32>,
    pub derivation_id: Option<i32>,
    pub min: Option<f32>,
    pub max: Option<f32>,
    pub median: Option<f32>,
    pub footnote: Option<String>,
    pub min_year_acquired: Option<i32>,
    pub nutrient_id_nid: Option<i32>,
    pub nutrient_id_nnbr: Option<f32>,
}

#[derive(Identifiable, Queryable, PartialEq, Debug)]
#[table_name = "food_nutrient_derivation"]
pub struct FoodNutrientDerivation {
    pub id: i32,
    pub code: String,
    pub description: String,
    pub source_id: i32,
}

#[derive(Identifiable, Queryable, PartialEq, Debug)]
#[table_name = "nutrient"]
pub struct Nutrient {
    pub id: i32,
    pub name: String,
    pub unit_name: String,
    pub nutrient_nbr: f32,
    pub rank: String,
}

#[derive(Identifiable, Queryable, PartialEq, Debug)]
#[table_name = "food_nutrient_source"]
pub struct FoodNutrientSource {
    pub id: i32,
    pub code: i32,
    pub description: String,
}
