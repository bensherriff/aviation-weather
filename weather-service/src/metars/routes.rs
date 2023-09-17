use crate::{error_handler::ServiceError, db::Metadata};
use crate::metars::QueryMetar;
use actix_web::{get, web, HttpResponse, Responder};
use log::error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MetarsResponse {
    pub data: Vec<QueryMetar>,
    pub meta: Metadata
}

#[get("metars/{ids}")]
async fn get_all(ids: web::Path<String>) -> impl Responder {
    let airports = match web::block(|| Ok::<_, ServiceError>(async {QueryMetar::get_all(ids.into_inner()).await}))
        .await
        .unwrap()
        .unwrap()
        .await {
            Ok(a) => a,
            Err(err) => {
                error!("{}", err);
                return err.to_http_response();
            }
        };
    HttpResponse::Ok().json(MetarsResponse {
        data: airports,
        meta: Metadata { page: 0, limit: 0, pages: 0, total: 0 }
    })
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(get_all);
}