use crate::airports::{Airport, Airports};
use crate::error_handler::CustomError;
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;

#[get("/airports")]
async fn find_all() -> Result<HttpResponse, CustomError> {
  let airports = web::block(|| Airports::find_all()).await.unwrap();
    Ok(HttpResponse::Ok().json(airports))
}

#[get("/airports/{id}")]
async fn find(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let airport = Airports::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(airport))
}

#[post("/airports")]
async fn create(airport: web::Json<Airport>) -> Result<HttpResponse, CustomError> {
    let airport = Airports::create(airport.into_inner())?;
    Ok(HttpResponse::Ok().json(airport))
}

#[put("/airports/{id}")]
async fn update(
    id: web::Path<i32>,
    airport: web::Json<Airport>,
) -> Result<HttpResponse, CustomError> {
    let airport = Airports::update(id.into_inner(), airport.into_inner())?;
    Ok(HttpResponse::Ok().json(airport))
}

#[delete("/airports/{id}")]
async fn delete(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let deleted_airport = Airports::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_airport })))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
  config.service(find_all);
  config.service(find);
  config.service(create);
  config.service(update);
  config.service(delete);
}