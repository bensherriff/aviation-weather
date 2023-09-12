use crate::db;
use crate::error_handler::CustomError;
use crate::schema::airports;
use diesel::prelude::*;
use postgis_diesel::types::*;
use postgis_diesel::functions::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[diesel(table_name = airports)]
pub struct Airport {
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

#[derive(Serialize, Deserialize, Queryable)]
pub struct Airports {
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

impl Airports {
  pub fn find_all(bounds: Option<Polygon<Point>>, limit: i32, page: i32) -> Result<Vec<Self>, CustomError> {
    let mut conn = db::connection()?;
    let airports = airports::table
      .limit(limit as i64)
      .filter(airports::id.gt(page * limit).and(match bounds {
        Some(b) => st_contains(b, airports::point),
        None => {
          let polygon: Polygon<Point> = Polygon::new(Some(4326));
          st_contains(polygon, airports::point)
        }
      }))
      .load::<Airports>(&mut conn)?;
    Ok(airports)
  }

  pub fn find(icao: String) -> Result<Self, CustomError> {
    let mut conn = db::connection()?;
    let airport = airports::table.filter(airports::icao.eq(icao)).first(&mut conn)?;
    Ok(airport)
}

pub fn create(airport: Airport) -> Result<Self, CustomError> {
    let mut conn = db::connection()?;
    let airport = Airport::from(airport);
    let airport = diesel::insert_into(airports::table)
        .values(airport)
        .get_result(&mut conn)?;
    Ok(airport)
}

pub fn update(id: i32, airport: Airport) -> Result<Self, CustomError> {
    let mut conn = db::connection()?;
    let airport = diesel::update(airports::table)
        .filter(airports::id.eq(id))
        .set(airport)
        .get_result(&mut conn)?;
    Ok(airport)
}

pub fn delete(id: i32) -> Result<usize, CustomError> {
    let mut conn = db::connection()?;
    let res = diesel::delete(airports::table.filter(airports::id.eq(id))).execute(&mut conn)?;
    Ok(res)
}
}