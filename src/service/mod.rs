use actix_web::{error, web};
use derive_more::{Display, Error};

use crate::db::DB;
use crate::nutritionix::NutritionixService;

#[derive(Debug, Display, Error)]
#[display(fmt = "{}", err)]
pub struct ServiceError {
    err: anyhow::Error,
}

impl From<anyhow::Error> for ServiceError {
    fn from(err: anyhow::Error) -> Self {
        Self { err }
    }
}

impl error::ResponseError for ServiceError {}

pub struct AppState {
    pub db: DB,
    pub nixservice: NutritionixService,
}

mod rest;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/v1").configure(rest::config));
}
