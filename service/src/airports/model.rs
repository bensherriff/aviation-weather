use crate::db;
use crate::error_handler::ServiceError;
use crate::schema::airports;
use diesel::dsl::count_star;
use diesel::prelude::*;
// use log::trace;
use postgis_diesel::types::*;
use postgis_diesel::functions::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[diesel(table_name = airports)]
pub struct InsertAirport {
  pub icao: String,
  pub category: String,
  pub full_name: String,
  pub elevation_ft: Option<i32>,
  pub continent: String,
  pub iso_country: String,
  pub iso_region: String,
  pub municipality: String,
  pub gps_code: String,
  pub iata_code: String,
  pub local_code: String,
  pub point: Point
}

#[derive(Serialize, Deserialize, Queryable, QueryableByName)]
#[diesel(table_name = airports)]
pub struct QueryAirport {
  pub icao: String,
  pub id: i32,
  pub category: String,
  pub full_name: String,
  pub elevation_ft: Option<i32>,
  pub continent: String,
  pub iso_country: String,
  pub iso_region: String,
  pub municipality: String,
  pub gps_code: String,
  pub iata_code: String,
  pub local_code: String,
  pub point: Point
}

impl QueryAirport {
  pub fn get_all(bounds: &Option<Polygon<Point>>, category: &Option<String>, filter: &Option<String>, limit: i32, page: i32) -> Result<Vec<Self>, ServiceError> {
    let mut conn = db::connection()?;
    
    let mut query = airports::table
      .limit(limit as i64)
      .into_boxed();
    query = query.filter(airports::id.gt(std::cmp::max(1, page - 1) * limit));

    if let Some(bounds) = bounds {
      query = query.filter(st_contains(bounds, airports::point));
    }
    if let Some(category) = category {
      query = query.filter(airports::category.eq(category));
    }
    if let Some(filter) = filter {
      query = query.filter(airports::icao
        .ilike(format!("%{}%", filter))
        .or(airports::full_name.ilike(format!("%{}%", filter)))
      )
    }
    let airports: Vec<QueryAirport> = query.order((airports::id.asc(), airports::category.asc())).load::<QueryAirport>(&mut conn)?;
    Ok(airports)
  }

  pub fn get_count(bounds: &Option<Polygon<Point>>, category: &Option<String>, filter: &Option<String>) -> Result<i64, ServiceError> {
    let mut conn = db::connection()?;
    let mut query = airports::table.select(count_star()).into_boxed();

    if let Some(bounds) = bounds {
      query = query.filter(st_contains(bounds, airports::point));
    }
    if let Some(category) = category {
      query = query.filter(airports::category.eq(category));
    }
    if let Some(filter) = filter {
      query = query.filter(airports::icao
        .ilike(format!("%{}%", filter))
        .or(airports::full_name.ilike(format!("%{}%", filter)))
      )
    }

    let count: i64 = query.first(&mut conn)?;
    return Ok(count);
  }

  pub fn find(icao: String) -> Result<Self, ServiceError> {
    let mut conn = db::connection()?;
    let airport = airports::table.filter(airports::icao.eq(icao)).first(&mut conn)?;
    Ok(airport)
}

pub fn create(airport: InsertAirport) -> Result<Self, ServiceError> {
    let mut conn = db::connection()?;
    let airport = InsertAirport::from(airport);
    let airport = diesel::insert_into(airports::table)
        .values(airport)
        .get_result(&mut conn)?;
    Ok(airport)
}

pub fn update(id: i32, airport: InsertAirport) -> Result<Self, ServiceError> {
    let mut conn = db::connection()?;
    let airport = diesel::update(airports::table)
        .filter(airports::id.eq(id))
        .set(airport)
        .get_result(&mut conn)?;
    Ok(airport)
}

pub fn delete(id: i32) -> Result<usize, ServiceError> {
    let mut conn = db::connection()?;
    let res = diesel::delete(airports::table.filter(airports::id.eq(id))).execute(&mut conn)?;
    Ok(res)
}
}