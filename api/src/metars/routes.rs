use crate::error::Error;
use crate::metars::Metar;
use actix_web::{get, web, HttpResponse, HttpRequest};
use log::error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct FindAllParameters {
  icaos: Option<String>,
}

#[get("metars")]
async fn find_all(req: HttpRequest) -> HttpResponse {
  let parameters = web::Query::<FindAllParameters>::from_query(req.query_string()).unwrap();
  let icao_option = &parameters.icaos;
  let icao_string = match icao_option {
    Some(i) => i,
    None => return HttpResponse::UnprocessableEntity().body("Missing icaos parameter"),
  };
  let icaos: Vec<&str> = icao_string.split(',').collect();

  let metars = match Metar::find_all(&icaos).await {
    Ok(a) => a,
    Err(err) => {
      error!("{}", err);
      return err.to_http_response();
    }
  };
  HttpResponse::Ok().json(metars)
}

pub fn init_routes(config: &mut web::ServiceConfig) {
  config.service(find_all);
}
