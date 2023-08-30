use crate::db;
use crate::error_handler::CustomError;
use crate::schema::airports;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "airports"]
pub struct Airport {
  pub name: String,
  pub icao: String,
  pub latitude: f32,
  pub longitude: f32,
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct Airports {
  pub id: i32,
  pub name: String,
  pub icao: String,
  pub latitude: f32,
  pub longitude: f32,
}

impl Airports {
  pub fn find_all() -> Result<Vec<Self>, CustomError> {
    let conn = db::connection()?;
    let airports = airports::table.load::<Airports>(&conn)?;
    Ok(airports)
  }
}