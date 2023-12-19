use std::str::FromStr;

use crate::db;
use crate::error_handler::ServiceError;
use crate::db::schema::airports;
use diesel::prelude::*;
use diesel::sql_query;
use log::error;
use postgis_diesel::types::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Airport {
  pub icao: String,
  pub category: String,
  pub name: String,
  pub elevation_ft: f32,
  pub iso_country: String,
  pub iso_region: String,
  pub municipality: String,
  pub iata_code: String,
  pub local_code: String,
  pub latitude: f64,
  pub longitude: f64,
  pub has_tower: Option<bool>,
}

impl Into<QueryAirport> for Airport {
  fn into(self) -> QueryAirport {
    return QueryAirport {
      icao: self.icao.clone(),
      category: self.category.clone(),
      name: self.name.clone(),
      elevation_ft: self.elevation_ft,
      iso_country: self.iso_country.clone(),
      iso_region: self.iso_region.clone(),
      municipality: self.municipality.clone(),
      iata_code: self.iata_code.clone(),
      local_code: self.local_code.clone(),
      point: Point::new(self.longitude, self.latitude, Some(4326)),
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
  pub name: String,
  pub elevation_ft: f32,
  pub iso_country: String,
  pub iso_region: String,
  pub municipality: String,
  pub iata_code: String,
  pub local_code: String,
  pub point: Point,
  pub data: serde_json::Value
}

#[derive(Debug)]
pub struct QueryFilters {
  pub search: Option<String>,
  pub bounds: Option<Polygon<Point>>,
  pub categories: Option<Vec<String>>,
  pub order_field: Option<QueryOrderField>,
  pub order_by: Option<QueryOrderBy>
}

impl Default for QueryFilters {
  fn default() -> Self {
    QueryFilters {
      search: None,
      bounds: None,
      categories: None,
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
      "iata_code" => Ok(QueryOrderField::Iata),
      "local_code" => Ok(QueryOrderField::Local),
      _ => Err(())
    }
  }
}

impl QueryAirport {
  pub fn get_all(filters: &QueryFilters, limit: i32, page: i32) -> Result<Vec<Self>, ServiceError> {
    let mut conn = db::connection()?;
    let mut query: String = "SELECT * FROM airports".to_string();
    query = format!("{} {}", query, QueryAirport::build_filter_query(&filters)?);

    if let Some(order_by) = &filters.order_by {
      match order_by {
        QueryOrderBy::Asc => {
          if let Some(order_field) = &filters.order_field {
            query = match order_field {
              QueryOrderField::Icao => format!("{} ORDER BY icao ASC", query),
              QueryOrderField::Name => format!("{} ORDER BY name ASC", query),
              QueryOrderField::Category => format!("{} ORDER BY category ASC", query),
              QueryOrderField::Country => format!("{} ORDER BY iso_country ASC", query),
              QueryOrderField::Region => format!("{} ORDER BY iso_region ASC", query),
              QueryOrderField::Municipality => format!("{} ORDER BY municipality ASC", query),
              QueryOrderField::Iata => format!("{} ORDER BY iata_code ASC", query),
              QueryOrderField::Local => format!("{} ORDER BY local_code ASC", query),
            };
          };
        },
        QueryOrderBy::Desc => {
          if let Some(order_field) = &filters.order_field {
            query = match order_field {
              QueryOrderField::Icao => format!("{} ORDER BY icao DESC", query),
              QueryOrderField::Name => format!("{} ORDER BY name DESC", query),
              QueryOrderField::Category => format!("{} ORDER BY category DESC", query),
              QueryOrderField::Country => format!("{} ORDER BY iso_country DESC", query),
              QueryOrderField::Region => format!("{} ORDER BY iso_region DESC", query),
              QueryOrderField::Municipality => format!("{} ORDER BY municipality DESC", query),
              QueryOrderField::Iata => format!("{} ORDER BY iata_code DESC", query),
              QueryOrderField::Local => format!("{} ORDER BY local_code DESC", query),
            };
          };
        }
      }
    }
    // Limit query to page and limit
    query = format!("{} LIMIT {} OFFSET {}", query, limit, (page - 1) * limit);

    let airports: Vec<QueryAirport> = match sql_query(query).load(&mut conn) {
      Ok(a) => a,
      Err(err) => return Err(ServiceError { status: 500, message: format!("{}", err) })
    };
    Ok(airports)
  }

  pub fn get_count(filters: &QueryFilters) -> Result<i64, ServiceError> {
    let mut conn = db::connection()?;
    let mut query = "SELECT COUNT(*) FROM airports".to_string();
    query = format!("{} {}", query, QueryAirport::build_filter_query(&filters)?);

    let count: i64 = match sql_query(query).execute(&mut conn) {
      Ok(c) => c as i64,
      Err(err) => return Err(ServiceError { status: 500, message: format!("{}", err) })
    };
    return Ok(count);
  }

  // TODO: Unsafe query, need to sanitize inputs
  fn build_filter_query(filters: &QueryFilters) -> Result<String, ServiceError> {
    let mut query = "".to_string();
    let mut parts: Vec<String> = vec![];

    if let Some(bounds) = &filters.bounds {
      // convert bounds to a WKT polygon
      if bounds.rings.len() > 1 {
        return Err(ServiceError { status: 400, message: "Only one polygon is allowed".to_string() })
      } else {
        let mut points: Vec<String> = vec![];
        bounds.rings.iter().for_each(|ring| {
          ring.iter().for_each(|point| {
            points.push(format!("{} {}", point.get_x(), point.get_y()));
          });
        });
        let bounds = format!("POLYGON(({}))", points.join(","));
        parts.push(format!("ST_Contains(ST_GeomFromText('{}', 4326), point)", bounds));
      }
    }
    if let Some(categories) = &filters.categories {
      parts.push(format!("({})", categories.iter().map(|category| format!("category = '{}'", category)).collect::<Vec<String>>().join(" OR ")));
    }
    if let Some(search) = &filters.search {
      let search_strs = vec!["icao", "name", "iso_country", "iso_region", "municipality", "iata_code", "local_code"];
      parts.push(format!("({})", search_strs.iter().map(|s| format!("{} ILIKE '%{}%'", s, search)).collect::<Vec<String>>().join(" OR ")));
    }

    if parts.len() > 0 {
      query = format!("{} WHERE {}", query, parts.join(" AND "));
    }

    return Ok(query);
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

  pub fn insert_all (airports: Vec<Self>) -> Result<Vec<Self>, ServiceError> {
    let mut conn: r2d2::PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>> = db::connection()?;
    let mut inserted_airports: Vec<Self> = vec![];
    for airport in airports {
      let airport = Self::from(airport);
      let airport = diesel::insert_into(airports::table)
          .values(airport)
          .get_result(&mut conn)?;
      inserted_airports.push(airport);
    }
    Ok(inserted_airports)
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
