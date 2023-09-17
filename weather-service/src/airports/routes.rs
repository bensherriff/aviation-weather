use crate::{airports::{InsertAirport, QueryAirport}, db::{self, Metadata}};
use actix_web::{delete, get, post, put, web, HttpResponse, HttpRequest};
use log::{error, warn};
use postgis_diesel::types::{Polygon, Point};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct GetAllParameters {
  filter: Option<String>,
  bounds: Option<String>,
  category: Option<String>,
  limit: i32,
  page: i32
}

#[get("/import")]
async fn import() -> HttpResponse {
  db::import_data();
  HttpResponse::Ok().body({})
}

#[derive(Serialize, Deserialize)]
pub struct AirportsResponse {
    pub data: Vec<QueryAirport>,
    pub meta: Metadata
}

#[get("/airports")]
async fn get_all(req: HttpRequest) -> HttpResponse {
  let params = web::Query::<GetAllParameters>::from_query(req.query_string()).unwrap();
  let polygon: Option<Polygon<Point>> = match &params.bounds {
    Some(b) => {
      let bounds: Vec<&str> = b.split(",").collect();
      if bounds.len() != 4 {
        warn!("Expected 4 bounds, received {}: {}", bounds.len(), b);
        return HttpResponse::UnprocessableEntity().body(format!("Received {}; expected NE_LAT,NE_LON,SW_LAT,SW_LON", b))
      }
      let ne_lat = match bounds[0].parse::<f64>() {
        Ok(b) => b,
        Err(err) => {
          warn!("{}", err);
          return HttpResponse::UnprocessableEntity().body(format!("{}", err))
        }
      };
      let ne_lon = match bounds[1].parse::<f64>() {
        Ok(b) => b,
        Err(err) => {
          warn!("{}", err);
          return HttpResponse::UnprocessableEntity().body(format!("{}", err))
        }
      };
      let sw_lat = match bounds[2].parse::<f64>() {
        Ok(b) => b,
        Err(err) => {
          warn!("{}", err);
          return HttpResponse::UnprocessableEntity().body(format!("{}", err))
        }
      };
      let sw_lon = match bounds[3].parse::<f64>() {
        Ok(b) => b,
        Err(err) => {
          warn!("{}", err);
          return HttpResponse::UnprocessableEntity().body(format!("{}", err))
        }
      };
      let mut polygon: Polygon<Point> = Polygon::new(Some(4326));
      polygon.add_point(Point { x: sw_lon, y: sw_lat, srid: Some(4326) });
      polygon.add_point(Point { x: ne_lon, y: sw_lat, srid: Some(4326) });
      polygon.add_point(Point { x: ne_lon, y: ne_lat, srid: Some(4326) });
      polygon.add_point(Point { x: sw_lon, y: ne_lat, srid: Some(4326) });
      polygon.add_point(Point { x: sw_lon, y: sw_lat, srid: Some(4326) });
      Some(polygon)
    },
    None => None
  };
  let category = match &params.category {
    Some(c) => Some(c.to_string()),
    None => None
  };
  let filter = match &params.filter {
    Some(f) => Some(f.to_string()),
    None => None
  };

  match web::block(move || QueryAirport::get_all(polygon, category, filter, params.limit, params.page)).await.unwrap() {
    Ok(a) => HttpResponse::Ok().json(AirportsResponse {
      data: a,
      meta: Metadata { page: 0, limit: 0, pages: 0, total: 0 }
    }),
    Err(err) => {
      error!("{}", err);
      err.to_http_response()
    }
  }
}

#[derive(Serialize, Deserialize)]
pub struct AirportResponse {
    pub data: QueryAirport,
    pub meta: Metadata
}

#[get("/airports/{icao}")]
async fn get(icao: web::Path<String>) -> HttpResponse {
  match QueryAirport::find(icao.into_inner()) {
    Ok(a) => HttpResponse::Ok().json(AirportResponse {
      data: a,
      meta: Metadata { page: 0, limit: 0, pages: 0, total: 0 }
    }),
    Err(err) => {
      error!("{}", err);
      err.to_http_response()
    }
  }
}

#[post("/airports")]
async fn create(airport: web::Json<InsertAirport>) -> HttpResponse {
  match QueryAirport::create(airport.into_inner()) {
    Ok(a) => HttpResponse::Created().json(a),
    Err(err) => {
      error!("{}", err);
      err.to_http_response()
    }
  }
}

#[put("/airports/{icao}")]
async fn update(icao: web::Path<i32>, airport: web::Json<InsertAirport>) -> HttpResponse {
  match QueryAirport::update(icao.into_inner(), airport.into_inner()) {
    Ok(a) => HttpResponse::Ok().json(a),
    Err(err) => {
      error!("{}", err);
      err.to_http_response()
    }
  }
}

#[delete("/airports/{icao}")]
async fn delete(icao: web::Path<i32>) -> HttpResponse {
  match QueryAirport::delete(icao.into_inner()) {
    Ok(_) => HttpResponse::NoContent().finish(),
    Err(err) => {
      error!("{}", err);
      err.to_http_response()
    }
  }
}

pub fn init_routes(config: &mut web::ServiceConfig) {
  config.service(get_all);
  config.service(get);
  config.service(create);
  config.service(update);
  config.service(delete);
  config.service(import);
}