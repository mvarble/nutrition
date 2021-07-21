#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;
embed_migrations!("./migrations");

#[macro_use]
extern crate serde;

#[macro_use]
extern crate lazy_static;

mod db;
mod env;
mod nutritionix;
mod schema;
mod service;
mod si;

use actix_web::{error::Result, middleware, web, App, HttpResponse, HttpServer};
use db::DB;
use nutritionix::NutritionixService;
use service::ServiceError;

#[actix_web::main]
async fn main() -> Result<()> {
    let env = env::get().map_err(ServiceError::from)?;
    let db = DB::new(env.database_url).map_err(ServiceError::from)?;
    let nixservice = NutritionixService::new(env.nutritionix_app_id, env.nutritionix_app_key);
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .data(service::AppState {
                db: db.clone(),
                nixservice: nixservice.clone(),
            })
            .configure(service::config)
            .default_service(web::route().to(|| {
                HttpResponse::NotFound()
                    .content_type("text/plain; charset=utf-8")
                    .body("PATH NOT FOUND")
            }))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;
    Ok(())
}
