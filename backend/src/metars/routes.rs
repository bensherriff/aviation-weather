use crate::error_handler::CustomError;
use crate::metars::Metars;
use actix_web::{get, web, HttpResponse};

#[get("metars/{ids}")]
async fn get_all(ids: web::Path<String>) -> Result<HttpResponse, CustomError> {
    let airports = web::block(|| Ok::<_, CustomError>(async {Metars::get_all(ids.into_inner()).await}))
        .await
        .unwrap()
        .unwrap()
        .await
        .unwrap();
    Ok(HttpResponse::Ok().json(airports))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(get_all);
}