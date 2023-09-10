use crate::{airports::{Airport, Airports}, db};
use actix_web::{delete, get, post, put, web, HttpResponse, HttpRequest};
use log::error;
use postgis_diesel::types::{Polygon, Point};
use serde::{Serialize, Deserialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
struct FindAllParams {
  ne_lat: f64,
  ne_lon: f64,
  sw_lat: f64,
  sw_lon: f64,
  limit: i32,
  page: i32
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
struct Coordinate {
  lon: f64,
  lat: f64
}

#[get("/setup")]
async fn setup() -> HttpResponse {
  db::import_data();
  HttpResponse::Ok().finish()
}

#[get("/airports")]
async fn find_all(req: HttpRequest) -> HttpResponse {
  let params = web::Query::<FindAllParams>::from_query(req.query_string()).unwrap();
  let mut polygon: Polygon<Point> = Polygon::new(Some(4326));
  polygon.add_point(Point { x: params.sw_lon, y: params.sw_lat, srid: Some(4326) });
  polygon.add_point(Point { x: params.ne_lon, y: params.sw_lat, srid: Some(4326) });
  polygon.add_point(Point { x: params.ne_lon, y: params.ne_lat, srid: Some(4326) });
  polygon.add_point(Point { x: params.sw_lon, y: params.ne_lat, srid: Some(4326) });
  polygon.add_point(Point { x: params.sw_lon, y: params.sw_lat, srid: Some(4326) });
  match web::block(move || Airports::find_all(polygon, params.limit, params.page)).await.unwrap() {
    Ok(a) => HttpResponse::Ok().json(a),
    Err(err) => {
      error!("{}", err);
      HttpResponse::InternalServerError().finish()
    }
  }
}

#[get("/airports/{id}")]
async fn find(id: web::Path<i32>) -> HttpResponse {
  match Airports::find(id.into_inner()) {
    Ok(a) => HttpResponse::Ok().json(a),
    Err(err) => {
      error!("{}", err);
      HttpResponse::InternalServerError().finish()
    }
  }
}

#[post("/airports")]
async fn create(airport: web::Json<Airport>) -> HttpResponse {
  match Airports::create(airport.into_inner()) {
    Ok(a) => HttpResponse::Ok().json(a),
    Err(err) => {
      error!("{}", err);
      HttpResponse::InternalServerError().finish()
    }
  }
}

#[put("/airports/{id}")]
async fn update(id: web::Path<i32>, airport: web::Json<Airport>) -> HttpResponse {
  match Airports::update(id.into_inner(), airport.into_inner()) {
    Ok(a) => HttpResponse::Ok().json(a),
    Err(err) => {
      error!("{}", err);
      HttpResponse::InternalServerError().finish()
    }
  }
}

#[delete("/airports/{id}")]
async fn delete(id: web::Path<i32>) -> HttpResponse {
  match Airports::delete(id.into_inner()) {
    Ok(a) => HttpResponse::Ok().json(json!({ "deleted": a })),
    Err(err) => {
      error!("{}", err);
      HttpResponse::InternalServerError().finish()
    }
  }
}

pub fn init_routes(config: &mut web::ServiceConfig) {
  config.service(find_all);
  config.service(find);
  config.service(create);
  config.service(update);
  config.service(delete);
  config.service(setup);
}