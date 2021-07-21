use actix_web::{client::Client, http, web, HttpResponse, Result};

use super::{AppState, ServiceError};
use crate::db::DB;

async fn upc(
    query: web::Query<serde_json::Value>,
    data: web::Data<AppState>,
) -> Result<HttpResponse> {
    if let Some(upc) = query.get("upc").and_then(|upc| upc.as_str()) {
        // valid url
        let db = &data.db;
        let conn = db.connect().map_err(ServiceError::from)?;
        let foods = DB::get_foods_by_upcs(&conn, &[upc.into()]).map_err(ServiceError::from)?;
        if let Some(food) = foods.get(0) {
            // food already in database
            Ok(HttpResponse::Ok().json(food))
        } else {
            // if food not in database, request nutritionix
            let nixservice = &data.nixservice;
            let client = Client::default();
            let nixfoods = nixservice.request_upc(&client, upc).await?;
            let foods = DB::add_foods(&conn, nixfoods).map_err(ServiceError::from)?;
            if let Some(food) = foods.get(0) {
                // nutritionix succeeded; return food
                Ok(HttpResponse::Ok().json(food))
            } else {
                // nutritionix failed; no upc
                Ok(HttpResponse::Ok()
                    .content_type("text/plain; charset=utf-8")
                    .body("NO UPC MATCH"))
            }
        }
    } else {
        // bad url
        Ok(HttpResponse::Ok()
            .status(http::StatusCode::from_u16(422).unwrap())
            .content_type("text/plain; charset=utf-8")
            .body("NEED UPC QUERY"))
    }
}

async fn foods(
    payload: web::Json<serde_json::Value>,
    data: web::Data<AppState>,
) -> Result<HttpResponse> {
    if let Some(name) = payload.get("query").and_then(|name| name.as_str()) {
        // valid url
        let db = &data.db;
        let conn = db.connect().map_err(ServiceError::from)?;
        let foods = DB::get_foods_by_name(&conn, &name).map_err(ServiceError::from)?;
        Ok(HttpResponse::Ok().json(serde_json::json!({ "foods": foods })))
    } else {
        // bad url
        Ok(HttpResponse::Ok()
            .status(http::StatusCode::from_u16(422).unwrap())
            .content_type("text/plain; charset=utf-8")
            .body("NEED QUERY FIELD"))
    }
}

async fn natural(
    payload: web::Json<serde_json::Value>,
    data: web::Data<AppState>,
) -> Result<HttpResponse> {
    if let Some(name) = payload.get("query").and_then(|name| name.as_str()) {
        // valid url
        let nixservice = &data.nixservice;
        let client = Client::default();
        let nixfoods = nixservice.request_natural(&client, name).await?;
        let db = &data.db;
        let conn = db.connect().map_err(ServiceError::from)?;
        let foods = DB::add_foods(&conn, nixfoods).map_err(ServiceError::from)?;
        Ok(HttpResponse::Ok().json(serde_json::json!({ "foods": foods })))
    } else {
        // bad url
        Ok(HttpResponse::Ok()
            .status(http::StatusCode::from_u16(422).unwrap())
            .content_type("text/plain; charset=utf-8")
            .body("NEED QUERY FIELD"))
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    let callback = || {
        HttpResponse::Ok()
            .status(http::StatusCode::from_u16(405).unwrap())
            .json(serde_json::json!({ "error": "You must do a POST request to this address." }))
    };
    cfg.service(
        web::resource("/upc")
            .route(web::get().to(upc))
            .default_service(web::route().to(callback)),
    )
    .service(
        web::resource("/foods")
            .route(web::post().to(foods))
            .default_service(web::route().to(callback)),
    )
    .service(
        web::resource("/nutritionix")
            .route(web::post().to(natural))
            .default_service(web::route().to(callback)),
    );
}
