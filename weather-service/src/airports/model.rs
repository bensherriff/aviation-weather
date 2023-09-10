use crate::db;
use crate::error_handler::CustomError;
use crate::schema::airports;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "airports"]
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
  pub latitude: f64,
  pub longitude: f64,
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct Airports {
  pub id: i32,
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
  pub latitude: f64,
  pub longitude: f64,
}

impl Airports {
  pub fn find_all(limit: i32, page: i32) -> Result<Vec<Self>, CustomError> {
    let conn = db::connection()?;
    let airports = airports::table
      .limit(limit as i64)
      .filter(airports::id.gt(page * limit))
      .load::<Airports>(&conn)?;
    Ok(airports)
  }

  pub fn find(id: i32) -> Result<Self, CustomError> {
    let conn = db::connection()?;
    let airport = airports::table.filter(airports::id.eq(id)).first(&conn)?;
    Ok(airport)
}

pub fn create(airport: Airport) -> Result<Self, CustomError> {
    let conn = db::connection()?;
    let airport = Airport::from(airport);
    let airport = diesel::insert_into(airports::table)
        .values(airport)
        .get_result(&conn)?;
    Ok(airport)
}

pub fn update(id: i32, airport: Airport) -> Result<Self, CustomError> {
    let conn = db::connection()?;
    let airport = diesel::update(airports::table)
        .filter(airports::id.eq(id))
        .set(airport)
        .get_result(&conn)?;
    Ok(airport)
}

pub fn delete(id: i32) -> Result<usize, CustomError> {
    let conn = db::connection()?;
    let res = diesel::delete(airports::table.filter(airports::id.eq(id))).execute(&conn)?;
    Ok(res)
}
}