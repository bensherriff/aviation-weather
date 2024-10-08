use crate::{error::ApiError, db::Metadata};
use crate::metars::Metar;
use actix_web::{get, web, HttpResponse, HttpRequest};
use log::error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MetarsResponse {
  pub data: Vec<Metar>,
  pub meta: Metadata,
}

#[derive(Debug, Serialize, Deserialize)]
struct GetAllParameters {
  icaos: Option<String>,
}

#[get("metars")]
async fn get_all(req: HttpRequest) -> HttpResponse {
  let params = web::Query::<GetAllParameters>::from_query(req.query_string()).unwrap();
  let icao_option = params.icaos.clone();
  let icao_string = match icao_option {
    Some(i) => i,
    None => return HttpResponse::UnprocessableEntity().body("Missing icaos parameter"),
  };

  let metars =
    match web::block(|| Ok::<_, ApiError>(async { Metar::get_all(icao_string).await }))
      .await
      .unwrap()
      .unwrap()
      .await
    {
      Ok(a) => a,
      Err(err) => {
        error!("{}", err);
        return err.to_http_response();
      }
    };
  HttpResponse::Ok().json(metars)
}

pub fn init_routes(config: &mut web::ServiceConfig) {
  config.service(get_all);
}
