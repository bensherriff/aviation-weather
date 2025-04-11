use futures_util::stream::StreamExt as _;

use crate::{
  airports::Airport,
  db::Paged,
  auth::{Auth, verify_role},
};
use actix_multipart::Multipart;
use actix_web::{delete, get, post, put, web, HttpResponse, HttpRequest, ResponseError};
use crate::airports::{AirportQuery, UpdateAirport};
use crate::users::ADMIN_ROLE;

#[post("/import")]
async fn import_airports(mut payload: Multipart, auth: Auth) -> HttpResponse {
  if let Err(err) = verify_role(&auth, ADMIN_ROLE) {
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
          log::error!("Failed to get chunk: {}", err);
          return ResponseError::error_response(&err);
        }
      };
      bytes.extend_from_slice(&data);
    }

    // Convert bytes to Vec<Airport>
    let airports: Vec<Airport> = match serde_json::from_slice(&bytes) {
      Ok(a) => a,
      Err(err) => {
        log::error!("Failed to parse JSON: {}", err);
        return ResponseError::error_response(&err);
      }
    };

    match Airport::insert_all(airports).await {
      Ok(_) => {}
      Err(err) => return ResponseError::error_response(&err),
    };
  }
  HttpResponse::Ok().finish()
}

#[get("")]
async fn get_airports(req: HttpRequest) -> HttpResponse {
  let mut query = match web::Query::<AirportQuery>::from_query(req.query_string()) {
    Ok(q) => q.into_inner(),
    Err(err) => {
      log::error!("{}", err);
      AirportQuery::default()
    }
  };

  let total = Airport::count(&query).await;
  let page = query.page.unwrap_or(1);
  let mut limit = query.limit.unwrap_or(total as u32);
  if limit > 1000 {
    limit = 1000
  }
  query.limit = Some(limit);
  query.page = Some(page);

  match Airport::select_all(&query).await {
    Ok(airports) => HttpResponse::Ok().json(Paged {
      data: airports,
      page,
      limit,
      total,
    }),
    Err(err) => {
      log::error!("{}", err);
      ResponseError::error_response(&err)
    }
  }
}

#[get("/{icao}")]
async fn get_airport(icao: web::Path<String>, req: HttpRequest) -> HttpResponse {
  let metar = match web::Query::<AirportQuery>::from_query(req.query_string()) {
    Ok(q) => q.metars.unwrap_or_else(|| false),
    Err(err) => {
      log::error!("{}", err);
      false
    }
  };

  match Airport::select(&icao.into_inner(), metar).await {
    Some(airport) => HttpResponse::Ok().json(airport),
    None => HttpResponse::NotFound().finish(),
  }
}

#[post("")]
async fn insert_airport(airport: web::Json<Airport>, auth: Auth) -> HttpResponse {
  let _ = match verify_role(&auth, ADMIN_ROLE) {
    Ok(_) => {}
    Err(err) => return ResponseError::error_response(&err),
  };
  match airport.insert().await {
    Ok(a) => HttpResponse::Ok().json(a),
    Err(err) => {
      log::error!("{}", err);
      ResponseError::error_response(&err)
    }
  }
}

#[put("/{icao}")]
async fn update_airport(
  icao: web::Path<String>,
  airport: web::Json<UpdateAirport>,
  auth: Auth,
) -> HttpResponse {
  let _ = match verify_role(&auth, ADMIN_ROLE) {
    Ok(_) => {}
    Err(err) => return ResponseError::error_response(&err),
  };
  match Airport::update(&icao.into_inner(), &airport.into_inner()).await {
    Ok(a) => HttpResponse::Ok().json(a),
    Err(err) => {
      log::error!("{}", err);
      ResponseError::error_response(&err)
    }
  }
}

#[delete("")]
async fn delete_airports(auth: Auth) -> HttpResponse {
  let _ = match verify_role(&auth, ADMIN_ROLE) {
    Ok(_) => {}
    Err(err) => return ResponseError::error_response(&err),
  };
  match Airport::delete_all().await {
    Ok(_) => HttpResponse::NoContent().finish(),
    Err(err) => {
      log::error!("{}", err);
      ResponseError::error_response(&err)
    }
  }
}

#[delete("/{icao}")]
async fn delete_airport(icao: web::Path<String>, auth: Auth) -> HttpResponse {
  let _ = match verify_role(&auth, ADMIN_ROLE) {
    Ok(_) => {}
    Err(err) => return ResponseError::error_response(&err),
  };
  match Airport::delete(&icao.into_inner()).await {
    Ok(_) => HttpResponse::NoContent().finish(),
    Err(err) => {
      log::error!("{}", err);
      ResponseError::error_response(&err)
    }
  }
}

pub fn init_routes(config: &mut web::ServiceConfig) {
  config.service(
    web::scope("airports")
      .service(import_airports)
      .service(get_airports)
      .service(get_airport)
      .service(insert_airport)
      .service(update_airport)
      .service(delete_airports)
      .service(delete_airport),
  );
}
