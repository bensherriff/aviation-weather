use std::str::FromStr;
use futures_util::stream::StreamExt as _;

use crate::{
  airports::{QueryAirport, QueryFilters, QueryOrderField, QueryOrderBy, Airport, AirportCategory},
  db::{Response, Metadata},
  auth::{Auth, verify_role},
};
use actix_multipart::Multipart;
use actix_web::{delete, get, post, put, web, HttpResponse, HttpRequest, ResponseError};
use log::{error, warn};
use postgis_diesel::types::{Polygon, Point};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct AirportsQuery {
  icaos: Option<String>,
  name: Option<String>,
  bounds: Option<String>,
  categories: Option<String>,
  order_field: Option<String>,
  order_by: Option<String>,
  has_metar: Option<String>,
  limit: Option<i32>,
  page: Option<i32>,
}

#[post("/import")]
async fn import_airports(mut payload: Multipart, auth: Auth) -> HttpResponse {
  if let Err(err) = verify_role(&auth, "admin") {
    return ResponseError::error_response(&err);
  };

  while let Some(item) = payload.next().await {
    let mut bytes = web::BytesMut::new();
    let mut field = match item {
      Ok(field) => field,
      Err(err) => return ResponseError::error_response(&err),
    };

    // Build bytes from chunks
    while let Some(chunk) = field.next().await {
      let data = match chunk {
        Ok(data) => data,
        Err(err) => {
          error!("Failed to get chunk: {}", err);
          return ResponseError::error_response(&err);
        }
      };
      bytes.extend_from_slice(&data);
    }

    // Convert bytes to Vec<Airport>
    let airports: Vec<Airport> = match serde_json::from_slice(&bytes) {
      Ok(a) => a,
      Err(err) => {
        error!("Failed to parse JSON: {}", err);
        return ResponseError::error_response(&err);
      }
    };

    // Convert Vec<Airport> to Vec<QueryAirport> and insert into database
    let query_airports: Vec<QueryAirport> = airports.into_iter().map(|a| a.into()).collect();
    match QueryAirport::insert_all(query_airports) {
      Ok(_) => {}
      Err(err) => return ResponseError::error_response(&err),
    };
  }
  HttpResponse::Ok().finish()
}

#[get("")]
async fn get_airports(req: HttpRequest) -> HttpResponse {
  let params = web::Query::<AirportsQuery>::from_query(req.query_string()).unwrap();
  let mut filters = QueryFilters::default();
  filters.icaos = match &params.icaos {
    Some(i) => Some(i.split(",").map(|s| s.to_string()).collect()),
    None => None,
  };
  filters.name = params.name.clone();
  filters.categories = match &params.categories {
    Some(c) => Some(
      c.split(",")
        .map(|s| AirportCategory::from_str(s).unwrap())
        .collect(),
    ),
    None => None,
  };
  filters.bounds = match &params.bounds {
    Some(b) => {
      let bounds: Vec<&str> = b.split(",").collect();
      if bounds.len() != 4 {
        warn!("Expected 4 bounds, received {}: {}", bounds.len(), b);
        return HttpResponse::UnprocessableEntity().body(format!(
          "Received {}; expected NE_LAT,NE_LON,SW_LAT,SW_LON",
          b
        ));
      }
      let ne_lat = match bounds[0].parse::<f64>() {
        Ok(b) => b,
        Err(err) => {
          warn!("{}", err);
          return HttpResponse::UnprocessableEntity().body(format!("{}", err));
        }
      };
      let ne_lon = match bounds[1].parse::<f64>() {
        Ok(b) => b,
        Err(err) => {
          warn!("{}", err);
          return HttpResponse::UnprocessableEntity().body(format!("{}", err));
        }
      };
      let sw_lat = match bounds[2].parse::<f64>() {
        Ok(b) => b,
        Err(err) => {
          warn!("{}", err);
          return HttpResponse::UnprocessableEntity().body(format!("{}", err));
        }
      };
      let sw_lon = match bounds[3].parse::<f64>() {
        Ok(b) => b,
        Err(err) => {
          warn!("{}", err);
          return HttpResponse::UnprocessableEntity().body(format!("{}", err));
        }
      };
      let mut polygon: Polygon<Point> = Polygon::new(Some(4326));
      polygon.add_point(Point {
        x: sw_lon,
        y: sw_lat,
        srid: Some(4326),
      });
      polygon.add_point(Point {
        x: ne_lon,
        y: sw_lat,
        srid: Some(4326),
      });
      polygon.add_point(Point {
        x: ne_lon,
        y: ne_lat,
        srid: Some(4326),
      });
      polygon.add_point(Point {
        x: sw_lon,
        y: ne_lat,
        srid: Some(4326),
      });
      polygon.add_point(Point {
        x: sw_lon,
        y: sw_lat,
        srid: Some(4326),
      });
      Some(polygon)
    }
    None => None,
  };

  filters.order_by = match &params.order_by {
    Some(o) => Some(QueryOrderBy::from_str(&o).unwrap()),
    None => None,
  };
  filters.order_field = match &params.order_field {
    Some(o) => Some(QueryOrderField::from_str(&o).unwrap()),
    None => None,
  };
  filters.has_metar = match &params.has_metar {
    Some(h) => Some(h.parse::<bool>().unwrap()),
    None => None,
  };

  let limit = match params.limit {
    Some(l) => l,
    None => 100,
  };
  let page = match params.page {
    Some(p) => p,
    None => 1,
  };
  let total = match QueryAirport::get_count(&filters) {
    Ok(t) => t,
    Err(_) => 0,
  };

  match web::block(move || QueryAirport::get_all(&filters, limit, page))
    .await
    .unwrap()
  {
    Ok(a) => {
      // Convert Vec<QueryAirport> to Vec<Airport>
      let mut airports: Vec<Airport> = vec![];
      for airport in a {
        airports.push(airport.into());
      }
      HttpResponse::Ok().json(Response {
        data: airports,
        meta: Some(Metadata { page, limit, total }),
      })
    }
    Err(err) => {
      error!("{}", err);
      err.to_http_response()
    }
  }
}

#[get("/{icao}")]
async fn get_airport(icao: web::Path<String>) -> HttpResponse {
  match QueryAirport::get(&icao.into_inner()) {
    Ok(a) => {
      let airport: Airport = a.into();
      HttpResponse::Ok().json(airport)
    }
    Err(err) => {
      error!("{}", err);
      err.to_http_response()
    }
  }
}

#[post("")]
async fn create_airport(airport: web::Json<Airport>, auth: Auth) -> HttpResponse {
  let _ = match verify_role(&auth, "admin") {
    Ok(_) => {}
    Err(err) => return ResponseError::error_response(&err),
  };
  let query_airport: QueryAirport = airport.into_inner().into();
  match QueryAirport::insert(query_airport) {
    Ok(a) => {
      let airport: Airport = a.into();
      HttpResponse::Ok().json(airport)
    }
    Err(err) => {
      error!("{}", err);
      err.to_http_response()
    }
  }
}

#[put("/{icao}")]
async fn update_airport(
  _icao: web::Path<String>,
  airport: web::Json<Airport>,
  auth: Auth,
) -> HttpResponse {
  let _ = match verify_role(&auth, "admin") {
    Ok(_) => {}
    Err(err) => return ResponseError::error_response(&err),
  };
  let query_airport: QueryAirport = airport.into_inner().into();
  match QueryAirport::update(query_airport) {
    Ok(a) => {
      let airport: Airport = a.into();
      HttpResponse::Ok().json(airport)
    }
    Err(err) => {
      error!("{}", err);
      err.to_http_response()
    }
  }
}

#[delete("")]
async fn delete_airports(auth: Auth) -> HttpResponse {
  let _ = match verify_role(&auth, "admin") {
    Ok(_) => {}
    Err(err) => return ResponseError::error_response(&err),
  };
  match QueryAirport::delete(None) {
    Ok(_) => HttpResponse::NoContent().finish(),
    Err(err) => {
      error!("{}", err);
      err.to_http_response()
    }
  }
}

#[delete("/{icao}")]
async fn delete_airport(icao: web::Path<String>, auth: Auth) -> HttpResponse {
  let _ = match verify_role(&auth, "admin") {
    Ok(_) => {}
    Err(err) => return ResponseError::error_response(&err),
  };
  match QueryAirport::delete(Some(icao.into_inner())) {
    Ok(_) => HttpResponse::NoContent().finish(),
    Err(err) => {
      error!("{}", err);
      err.to_http_response()
    }
  }
}

pub fn init_routes(config: &mut web::ServiceConfig) {
  config.service(
    web::scope("airports")
      .service(import_airports)
      .service(get_airports)
      .service(get_airport)
      .service(create_airport)
      .service(update_airport)
      .service(delete_airports)
      .service(delete_airport),
  );
}
