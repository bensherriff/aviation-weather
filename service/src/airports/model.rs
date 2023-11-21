use std::str::FromStr;

use crate::db;
use crate::error_handler::ServiceError;
use crate::db::schema::airports;
use diesel::prelude::*;
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

#[derive(Debug)]
pub struct QueryFilters {
  pub search: Option<String>,
  pub bounds: Option<Polygon<Point>>,
  pub category: Option<String>,
  pub order_field: Option<QueryOrderField>,
  pub order_by: Option<QueryOrderBy>
}

impl Default for QueryFilters {
  fn default() -> Self {
    QueryFilters {
      search: None,
      bounds: None,
      category: None,
      order_field: None,
      order_by: None
    }
  }
}

#[derive(Debug)]
pub enum QueryOrderBy {
  Asc,
  Desc
}

impl FromStr for QueryOrderBy {
  type Err = ();
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "asc" => Ok(QueryOrderBy::Asc),
      "desc" => Ok(QueryOrderBy::Desc),
      _ => Err(())
    }
  }
}

#[derive(Debug)]
pub enum QueryOrderField {
  Icao,
  Name,
  Category,
  Continent,
  Country,
  Region,
  Municipality,
  GPS,
  Iata,
  Local,
}

impl FromStr for QueryOrderField {
  type Err = ();
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "icao" => Ok(QueryOrderField::Icao),
      "name" => Ok(QueryOrderField::Name),
      "category" => Ok(QueryOrderField::Category),
      "continent" => Ok(QueryOrderField::Continent),
      "iso_country" => Ok(QueryOrderField::Country),
      "iso_region" => Ok(QueryOrderField::Region),
      "municipality" => Ok(QueryOrderField::Municipality),
      "gps_code" => Ok(QueryOrderField::GPS),
      "iata_code" => Ok(QueryOrderField::Iata),
      "local_code" => Ok(QueryOrderField::Local),
      _ => Err(())
    }
  }
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
  pub fn get_all(filters: &QueryFilters, limit: i32, page: i32) -> Result<Vec<Self>, ServiceError> {
    let mut conn = db::connection()?;
    
    let mut query = airports::table.limit(limit as i64).into_boxed();
    // Limit query to page and limit
    let offset = (page - 1) * limit;
    query = query.offset(offset as i64);

    if let Some(bounds) = &filters.bounds {
      query = query.filter(st_contains(bounds, airports::point));
    }
    if let Some(category) = &filters.category {
      query = query.filter(airports::category.eq(category));
    }
    if let Some(search) = &filters.search {
      query = query.filter(
        airports::icao.ilike(format!("%{}%", search))
          .or(airports::full_name.ilike(format!("%{}%", search)))
          .or(airports::iso_country.ilike(format!("%{}%", search)))
          .or(airports::iso_region.ilike(format!("%{}%", search)))
          .or(airports::municipality.ilike(format!("%{}%", search)))
          .or(airports::gps_code.ilike(format!("%{}%", search)))
          .or(airports::iata_code.ilike(format!("%{}%", search)))
          .or(airports::local_code.ilike(format!("%{}%", search)))
      )
    }

    if let Some(order_by) = &filters.order_by {
      match order_by {
        QueryOrderBy::Asc => {
          if let Some(order_field) = &filters.order_field {
            query = match order_field {
              QueryOrderField::Icao => query.order(airports::icao.asc()),
              QueryOrderField::Name => query.order(airports::full_name.asc()),
              QueryOrderField::Category => query.order(airports::category.asc()),
              QueryOrderField::Continent => query.order(airports::continent.asc()),
              QueryOrderField::Country => query.order(airports::iso_country.asc()),
              QueryOrderField::Region => query.order(airports::iso_region.asc()),
              QueryOrderField::Municipality => query.order(airports::municipality.asc()),
              QueryOrderField::GPS => query.order(airports::gps_code.asc()),
              QueryOrderField::Iata => query.order(airports::iata_code.asc()),
              QueryOrderField::Local => query.order(airports::local_code.asc()),
            };
          };
        },
        QueryOrderBy::Desc => {
          if let Some(order_field) = &filters.order_field {
            query = match order_field {
              QueryOrderField::Icao => query.order(airports::icao.desc()),
              QueryOrderField::Name => query.order(airports::full_name.desc()),
              QueryOrderField::Category => query.order(airports::category.desc()),
              QueryOrderField::Continent => query.order(airports::continent.desc()),
              QueryOrderField::Country => query.order(airports::iso_country.desc()),
              QueryOrderField::Region => query.order(airports::iso_region.desc()),
              QueryOrderField::Municipality => query.order(airports::municipality.desc()),
              QueryOrderField::GPS => query.order(airports::gps_code.desc()),
              QueryOrderField::Iata => query.order(airports::iata_code.desc()),
              QueryOrderField::Local => query.order(airports::local_code.desc()),
            };
          };
        }
      }
    }
    let airports: Vec<QueryAirport> = query.load::<QueryAirport>(&mut conn)?;
    Ok(airports)
  }

  pub fn get_count(filters: &QueryFilters) -> Result<i64, ServiceError> {
    let mut conn = db::connection()?;
    let mut query = airports::table.count().into_boxed();

    if let Some(bounds) = &filters.bounds {
      query = query.filter(st_contains(bounds, airports::point));
    }
    if let Some(category) = &filters.category {
      query = query.filter(airports::category.eq(category));
    }
    if let Some(search) = &filters.search {
      query = query.filter(
        airports::icao.ilike(format!("%{}%", search))
          .or(airports::full_name.ilike(format!("%{}%", search)))
          .or(airports::iso_country.ilike(format!("%{}%", search)))
          .or(airports::iso_region.ilike(format!("%{}%", search)))
          .or(airports::municipality.ilike(format!("%{}%", search)))
          .or(airports::gps_code.ilike(format!("%{}%", search)))
          .or(airports::iata_code.ilike(format!("%{}%", search)))
          .or(airports::local_code.ilike(format!("%{}%", search)))
      )
    }

    let count: i64 = query.get_result(&mut conn)?;
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

  pub fn update(icao: String, airport: InsertAirport) -> Result<Self, ServiceError> {
    let mut conn = db::connection()?;
    let airport = diesel::update(airports::table)
        .filter(airports::icao.eq(icao))
        .set(airport)
        .get_result(&mut conn)?;
    Ok(airport)
  }

  pub fn delete(icao: Option<String>) -> Result<usize, ServiceError> {
    let mut conn = db::connection()?;
    let res = match icao {
      Some(icao) => diesel::delete(airports::table.filter(airports::icao.eq(icao))).execute(&mut conn)?,
      None => diesel::delete(airports::table).execute(&mut conn)?
    };
    Ok(res)
  }
}
