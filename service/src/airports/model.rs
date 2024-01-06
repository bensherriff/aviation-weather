use std::fmt::Display;
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
pub struct Runway {
  pub id: String,
  pub length_ft: f32,
  pub width_ft: f32,
  pub surface: String,
}

#[derive(Serialize, Deserialize)]
pub struct Frequency {
  pub id: String,
  pub frequency_mhz: f32,
}

#[derive(Serialize, Deserialize)]
pub struct Airport {
  pub icao: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub iata: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub local: Option<String>,
  pub name: String,
  pub category: AirportCategory,
  pub iso_country: String,
  pub iso_region: String,
  pub municipality: String,
  pub elevation_ft: f32,
  pub latitude: f64,
  pub longitude: f64,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub has_tower: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub has_beacon: Option<bool>,
  pub runways: Vec<Runway>,
  pub frequencies: Vec<Frequency>,
  pub has_metar: bool,
  pub public: bool,
}

impl Into<QueryAirport> for Airport {
  fn into(self) -> QueryAirport {
    return QueryAirport {
      icao: self.icao.clone(),
      category: self.category.clone().to_string(),
      name: self.name.clone(),
      elevation_ft: self.elevation_ft,
      iso_country: self.iso_country.clone(),
      iso_region: self.iso_region.clone(),
      municipality: self.municipality.clone(),
      has_metar: false,
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum AirportCategory {
  #[serde(rename = "small_airport")]
  Small,
  #[serde(rename = "medium_airport")]
  Medium,
  #[serde(rename = "large_airport")]
  Large,
  #[serde(rename = "heliport")]
  Heliport,
  #[serde(rename = "closed")]
  Closed,
  #[serde(rename = "seaplane_base")]
  Seaplane,
  #[serde(rename = "balloonport")]
  Balloonport,
  #[serde(rename = "unknown")]
  Unknown
}

impl FromStr for AirportCategory {
  type Err = ();
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "small_airport" => Ok(AirportCategory::Small),
      "medium_airport" => Ok(AirportCategory::Medium),
      "large_airport" => Ok(AirportCategory::Large),
      "heliport" => Ok(AirportCategory::Heliport),
      "closed" => Ok(AirportCategory::Closed),
      "seaplane_base" => Ok(AirportCategory::Seaplane),
      "balloonport" => Ok(AirportCategory::Balloonport),
      _ => Ok(AirportCategory::Unknown)
    }
  }
}

impl Display for AirportCategory {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      AirportCategory::Small => write!(f, "small_airport"),
      AirportCategory::Medium => write!(f, "medium_airport"),
      AirportCategory::Large => write!(f, "large_airport"),
      AirportCategory::Heliport => write!(f, "heliport"),
      AirportCategory::Closed => write!(f, "closed"),
      AirportCategory::Seaplane => write!(f, "seaplane_base"),
      AirportCategory::Balloonport => write!(f, "balloonport"),
      AirportCategory::Unknown => write!(f, "unknown")
    }
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
  pub has_metar: bool,
  pub point: Point,
  pub data: serde_json::Value
}

#[derive(Debug)]
pub struct QueryFilters {
  pub icaos: Option<Vec<String>>,
  pub name: Option<String>,
  pub bounds: Option<Polygon<Point>>,
  pub categories: Option<Vec<AirportCategory>>,
  pub order_field: Option<QueryOrderField>,
  pub order_by: Option<QueryOrderBy>
}

impl Default for QueryFilters {
  fn default() -> Self {
    QueryFilters {
      icaos: None,
      name: None,
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
      _ => Err(())
    }
  }
}

impl QueryAirport {
  pub fn get_all(filters: &QueryFilters, limit: i32, page: i32) -> Result<Vec<Self>, ServiceError> {
    let mut conn = db::connection()?;
    let mut query: String = "SELECT * FROM airports".to_string();
    query = format!("{} {}", query, QueryAirport::build_filter_query(&filters)?);

    query = format!("{} ORDER BY has_metar DESC", query);
    if let Some(order_by) = &filters.order_by {
      match order_by {
        QueryOrderBy::Asc => {
          if let Some(order_field) = &filters.order_field {
            query = match order_field {
              QueryOrderField::Icao => format!("{}, icao ASC", query),
              QueryOrderField::Name => format!("{}, name ASC", query),
              QueryOrderField::Category => format!("{}, category ASC", query),
              QueryOrderField::Country => format!("{}, iso_country ASC", query),
              QueryOrderField::Region => format!("{}, iso_region ASC", query),
              QueryOrderField::Municipality => format!("{}, municipality ASC", query),
            };
          };
        },
        QueryOrderBy::Desc => {
          if let Some(order_field) = &filters.order_field {
            query = match order_field {
              QueryOrderField::Icao => format!("{}, icao DESC", query),
              QueryOrderField::Name => format!("{}, name DESC", query),
              QueryOrderField::Category => format!("{}, category DESC", query),
              QueryOrderField::Country => format!("{}, iso_country DESC", query),
              QueryOrderField::Region => format!("{}, iso_region DESC", query),
              QueryOrderField::Municipality => format!("{}, municipality DESC", query),
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

    // TODO: Fix this to use get_result() instead of building this table to do the load()
    diesel::table! {
      airports (count) {
        count -> BigInt,
      }
    }
    #[derive(Debug, Queryable, QueryableByName)]
    #[diesel(table_name = airports)]
    struct Count {
      count: i64
    }

    let count: Vec<Count> = match sql_query(query).load(&mut conn) {
      Ok(a) => a,
      Err(err) => return Err(ServiceError { status: 500, message: format!("{}", err) })
    };
    return Ok(count[0].count);
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
      parts.push(format!("({})", categories.iter().map(|category| format!("category = '{}'", category.to_string())).collect::<Vec<String>>().join(" OR ")));
    }
    fn sanitize_icao(icao: &str) -> String {
      // Sanitize search to only allow [a-zA-Z0-9-\\s]
      icao.chars().filter(|c| c.is_alphanumeric() || *c == '-' || *c == ' ').collect::<String>()
    }
    if &filters.icaos.is_some() == &true && &filters.name.is_some() == &true {
      let icaos = filters.icaos.as_ref().unwrap();
      let name = sanitize_icao(filters.name.as_ref().unwrap());
      let icao_part = format!("({})", icaos.iter().map(|icao| format!("icao ILIKE '{}'", sanitize_icao(icao))).collect::<Vec<String>>().join(" OR "));
      let name_part = format!("name ILIKE '%{}%'", name);
      parts.push(format!("({} OR {})", icao_part, name_part));
    } else if let Some(icaos) = &filters.icaos {
      parts.push(format!("({})", icaos.iter().map(|icao| format!("icao ILIKE '{}'", sanitize_icao(icao))).collect::<Vec<String>>().join(" OR ")));
    } else if let Some(name) = &filters.name {
      let search = sanitize_icao(name);
      parts.push(format!("name ILIKE '%{}%'", search));
    }

    if parts.len() > 0 {
      query = format!("{} WHERE {}", query, parts.join(" AND "));
    }

    return Ok(query);
  }

  pub fn get(icao: &str) -> Result<Self, ServiceError> {
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

  pub fn update(airport: Self) -> Result<Self, ServiceError> {
    let mut conn = db::connection()?;
    let airport = diesel::update(airports::table)
        .filter(airports::icao.eq(airport.icao.clone()))
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
