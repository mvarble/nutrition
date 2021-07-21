#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;
embed_migrations!("./migrations");

#[macro_use]
extern crate serde;

#[macro_use]
extern crate lazy_static;

pub mod db;
pub mod env;
pub mod nutritionix;
pub mod schema;
pub mod service;
pub mod si;
