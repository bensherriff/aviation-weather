use std::str::FromStr;

use crate::db;
use crate::error_handler::ServiceError;
use crate::db::schema::airports;
use diesel::prelude::*;
use log::error;
use postgis_diesel::types::*;
use postgis_diesel::functions::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Airport {
  pub icao: String,
  pub category: String,
  pub full_name: String,
  pub elevation_ft: Option<i32>,
  pub iso_country: String,
  pub iso_region: String,
  pub municipality: String,
  pub gps_code: String,
  pub iata_code: String,
  pub local_code: String,
  pub point: Point,
  pub has_tower: Option<bool>,
}

impl Into<QueryAirport> for Airport {
  fn into(self) -> QueryAirport {
    return QueryAirport {
      icao: self.icao.clone(),
      category: self.category.clone(),
      full_name: self.full_name.clone(),
      iso_country: self.iso_country.clone(),
      iso_region: self.iso_region.clone(),
      municipality: self.municipality.clone(),
      gps_code: self.gps_code.clone(),
      iata_code: self.iata_code.clone(),
      local_code: self.local_code.clone(),
      point: self.point.clone(),
      data: match serde_json::to_value(&self) {
        Ok(d) => d,
        Err(err) => {
          error!("{}", err);
          serde_json::Value::Null
        }
      }
    }
  }
}

impl From<QueryAirport> for Airport {
  fn from(airport: QueryAirport) -> Self {
    serde_json::from_value(airport.data).unwrap()
  }
}

#[derive(Serialize, Deserialize, AsChangeset, Insertable, Queryable, QueryableByName)]
#[diesel(table_name = airports)]
pub struct QueryAirport {
  pub icao: String,
  pub category: String,
  pub full_name: String,
  pub iso_country: String,
  pub iso_region: String,
  pub municipality: String,
  pub gps_code: String,
  pub iata_code: String,
  pub local_code: String,
  pub point: Point,
  pub data: serde_json::Value
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

  pub fn insert(airport: Self) -> Result<Self, ServiceError> {
    let mut conn: r2d2::PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>> = db::connection()?;
    let airport = Self::from(airport);
    let airport = diesel::insert_into(airports::table)
        .values(airport)
        .get_result(&mut conn)?;
    Ok(airport)
  }

  pub fn update(icao: String, airport: Self) -> Result<Self, ServiceError> {
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
