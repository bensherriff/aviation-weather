use crate::metars::Metar;
use actix_web::{get, web, HttpResponse, HttpRequest};
use log::error;
use serde::{Deserialize, Serialize};
use crate::AppState;

#[derive(Debug, Serialize, Deserialize)]
struct FindAllParameters {
  icaos: Option<String>,
  force: Option<bool>,
}

#[get("metars")]
async fn find_all(data: web::Data<AppState>, req: HttpRequest) -> HttpResponse {
  let parameters = web::Query::<FindAllParameters>::from_query(req.query_string()).unwrap();
  let icao_option = &parameters.icaos;
  let icao_string = match icao_option {
    Some(i) => i,
    None => return HttpResponse::UnprocessableEntity().body("Missing icaos parameter"),
  };
  let icaos: Vec<String> = icao_string.split(',').map(|s| s.to_string()).collect();
  let force = &parameters.force.unwrap_or(false);

  let client = &data.client;
  let metars = match Metar::find_all(client, &icaos, force).await {
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
