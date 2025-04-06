use std::fmt::Display;
use std::str::FromStr;

use crate::db;
use log::error;
use serde::{Deserialize, Serialize};
use sqlx::postgres::types::PgPoint;
use crate::error::ApiResult;

const TABLE_NAME: &str = "airports";

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
struct RunwayDb {
  pub icao: String,
  pub id: String,
  pub length_ft: f32,
  pub width_ft: f32,
  pub surface: String,
}

#[derive(Debug)]
pub struct AirportFilter {
  pub icaos: Option<Vec<String>>,
  pub name: Option<String>,
  // pub bounds: Option<Polygon<Point>>,
  pub categories: Option<Vec<AirportCategory>>,
  pub has_metar: Option<bool>,
}

impl Default for AirportFilter {
  fn default() -> Self {
    AirportFilter {
      icaos: None,
      name: None,
      // bounds: None,
      categories: None,
      has_metar: None,
    }
  }
}

impl AirportDb {
  pub async fn find_all(_filter: &AirportFilter, _limit: i32, _page: i32) -> ApiResult<Vec<Self>> {
    let pool = db::pool();
    let airports: Vec<Self> = sqlx::query_as::<_, Self>(&format!(
      "SELECT * FROM {}",
      TABLE_NAME
    ))
      .fetch_all(pool)
      .await?;

    Ok(airports)
  }

  pub async fn count(_filter: &AirportFilter) -> ApiResult<i64> {
    let pool = db::pool();
    let count: i64 = sqlx::query_scalar::<_, i64>(&format!(
      "SELECT COUNT(*) FROM {}",
      TABLE_NAME
    ))
      .fetch_one(pool)
      .await?;

    Ok(count)
  }

  // fn build_query<'a>(
  //   mut query: QueryBuilder<'a, Postgres>,
  //   filter: &'a AirportFilter,
  // ) -> QueryBuilder<'a, Postgres> {
  //   if let Some(bounds) = &filter.bounds {
  //     // convert bounds to a WKT polygon
  //     if bounds.rings.len() > 1 {
  //       return Err(ApiError {
  //         status: 400,
  //         message: "Only one polygon is allowed".to_string(),
  //       });
  //     } else {
  //       let mut points: Vec<String> = vec![];
  //       bounds.rings.iter().for_each(|ring| {
  //         ring.iter().for_each(|point| {
  //           points.push(format!("{} {}", point.get_x(), point.get_y()));
  //         });
  //       });
  //       let bounds = format!("POLYGON(({}))", points.join(","));
  //       query.push(format!(
  //         "ST_Contains(ST_GeomFromText('{}', 4326), point)",
  //         bounds
  //       ));
  //     }
  //   }
  //   if let Some(categories) = &filter.categories {
  //     query.push(format!(
  //       "({})",
  //       categories
  //         .iter()
  //         .map(|category| format!("category = '{}'", category.to_string()))
  //         .collect::<Vec<String>>()
  //         .join(" OR ")
  //     ));
  //   }
  //
  //   fn sanitize_icao(icao: &str) -> String {
  //     // Sanitize search to only allow [a-zA-Z0-9-\\s]
  //     icao
  //       .chars()
  //       .filter(|c| c.is_alphanumeric() || *c == '-' || *c == ' ')
  //       .collect::<String>()
  //   }
  //
  //   if &filter.icaos.is_some() == &true && &filter.name.is_some() == &true {
  //     let icaos = filter.icaos.as_ref().unwrap();
  //     let name = sanitize_icao(filter.name.as_ref().unwrap());
  //     let icao_part = format!(
  //       "({})",
  //       icaos
  //         .iter()
  //         .map(|icao| format!("icao ILIKE '{}'", sanitize_icao(icao)))
  //         .collect::<Vec<String>>()
  //         .join(" OR ")
  //     );
  //     let name_part = format!("name ILIKE '%{}%'", name);
  //     parts.push(format!("({} OR {})", icao_part, name_part));
  //   } else if let Some(icaos) = &filter.icaos {
  //     parts.push(format!(
  //       "({})",
  //       icaos
  //         .iter()
  //         .map(|icao| format!("icao ILIKE '{}'", sanitize_icao(icao)))
  //         .collect::<Vec<String>>()
  //         .join(" OR ")
  //     ));
  //   } else if let Some(name) = &filter.name {
  //     let search = sanitize_icao(name);
  //     parts.push(format!("name ILIKE '%{}%'", search));
  //   }
  //   if let Some(has_metar) = &filter.has_metar {
  //     parts.push(format!("has_metar = {}", has_metar));
  //   }
  //
  //   if parts.len() > 0 {
  //     query = format!("{} WHERE {}", query, parts.join(" AND "));
  //   }
  //
  //   return Ok(query);
  // }

  pub async fn find_by_icao(icao: &str) -> ApiResult<Self> {
    let pool = db::pool();
    let airport =
      sqlx::query_as::<_, Self>(&format!("SELECT * FROM {} WHERE icao = $1", TABLE_NAME))
        .bind(icao)
        .fetch_one(pool)
        .await?;

    Ok(airport)
  }

  pub async fn insert(&self) -> ApiResult<()> {
    let pool = db::pool();
    sqlx::query(&format!(
      "INSERT INTO {} (
        icao,
        category,
        name,
        elevation_ft,
        iso_country,
        iso_region,
        municipality,
        has_metar,
        point,
        data
      ) VALUES (
        $1, $2, $3, $4, $5, $6, $7, $8, $9, $10
      )",
      TABLE_NAME
    ))
    .bind(self.icao.clone())
    .bind(self.category.clone())
    .bind(&self.name)
    .bind(self.elevation_ft)
    .bind(self.iso_country.clone())
    .bind(self.iso_region.clone())
    .bind(self.municipality.clone())
    .bind(self.has_metar.clone())
    // .bind(self.point.clone())
    .bind(self.data.clone())
    .execute(pool)
    .await?;
    Ok(())
  }

  // pub fn insert_vec(airports: Vec<Self>) -> ApiResult<Vec<Self>> {
  //   let mut conn: r2d2::PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>> =
  //     db::connection()?;
  //   let mut inserted_airports: Vec<Self> = vec![];
  //   for airport in airports {
  //     let airport = Self::from(airport);
  //     let airport = diesel::insert_into(airports::table)
  //       .values(airport)
  //       .on_conflict_do_nothing()
  //       .get_result(&mut conn)?;
  //     inserted_airports.push(airport);
  //   }
  //   Ok(inserted_airports)
  // }

  pub async fn update(&self) -> ApiResult<()> {
    // let mut conn = db::pool()?;
    // let airport = diesel::update(airports::table)
    //   .filter(airports::icao.eq(airport.icao.clone()))
    //   .set(airport)
    //   .get_result(&mut conn)?;
    // Ok(airport)
    Ok(())
  }

  pub async fn delete_all() -> ApiResult<()> {
    Ok(())
  }

  pub async fn delete_by_icao(_icao: &str) -> ApiResult<()> {
    // let mut conn = db::pool()?;
    // let res = match icao {
    //   Some(icao) => {
    //     diesel::delete(airports::table.filter(airports::icao.eq(icao))).execute(&mut conn)?
    //   }
    //   None => diesel::delete(airports::table).execute(&mut conn)?,
    // };
    // Ok(res)
    Ok(())
  }
}
