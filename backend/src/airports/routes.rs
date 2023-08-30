use crate::airports::{Airport, Airports};
use crate::error_handler::CustomError;
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;

#[get("/airports")]
async fn find_all() -> Result<HttpResponse, CustomError> {
  let airports = web::block(|| Airports::find_all()).await.unwrap();
    Ok(HttpResponse::Ok().json(airports))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
  config.service(find_all);
}