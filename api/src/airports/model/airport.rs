use std::str::FromStr;
use actix_web::web::Json;
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, QueryBuilder};
use crate::airports::model::airport_category::AirportCategory;
use crate::airports::{Frequency, Runway, UpdateFrequency, UpdateRunway};
use crate::db;
use crate::error::ApiResult;

const TABLE_NAME: &str = "airports";

#[derive(Debug, Serialize, Deserialize)]
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
  pub longitude: f32,
  pub latitude: f32,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub has_tower: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub has_beacon: Option<bool>,
  pub runways: Vec<Runway>,
  pub frequencies: Vec<Frequency>,
  pub public: bool,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
struct AirportRow {
  pub icao: String,
  pub iata: Option<String>,
  pub local: Option<String>,
  pub name: String,
  pub category: String,
  pub iso_country: String,
  pub iso_region: String,
  pub municipality: String,
  pub elevation_ft: f32,
  longitude: f32,
  latitude: f32,
  pub has_tower: Option<bool>,
  pub has_beacon: Option<bool>,
  pub public: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAirport {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub icao: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub iata: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub local: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub category: Option<AirportCategory>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub iso_country: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub iso_region: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub municipality: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub elevation_ft: Option<f32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub longitude: Option<f32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub latitude: Option<f32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub has_tower: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub has_beacon: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub runways: Option<Vec<UpdateRunway>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub frequencies: Option<Vec<UpdateFrequency>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub public: Option<bool>,
}

impl Into<AirportRow> for Airport {
  fn into(self) -> AirportRow {
    AirportRow {
      icao: self.icao.clone(),
      iata: self.iata.clone(),
      local: self.local.clone(),
      name: self.name.clone(),
      category: self.category.clone().to_string(),
      iso_country: self.iso_country.clone(),
      iso_region: self.iso_region.clone(),
      municipality: self.municipality.clone(),
      elevation_ft: self.elevation_ft,
      longitude: self.longitude,
      latitude: self.latitude,
      has_tower: self.has_tower,
      has_beacon: self.has_beacon,
      public: self.public,
    }
  }
}

impl From<AirportRow> for Airport {
  fn from(airport: AirportRow) -> Self {
    Airport {
      icao: airport.icao.clone(),
      iata: airport.iata.clone(),
      local: airport.local.clone(),
      name: airport.name.clone(),
      category: match AirportCategory::from_str(&airport.category) {
        Ok(c) => c,
        Err(_) => {
          log::error!("Invalid Airport category: {}", airport.category);
          AirportCategory::Unknown
        }
      },
      iso_country: airport.iso_country.clone(),
      iso_region: airport.iso_region.clone(),
      municipality: airport.municipality.clone(),
      elevation_ft: airport.elevation_ft,
      longitude: airport.longitude,
      latitude: airport.latitude,
      has_tower: airport.has_tower,
      has_beacon: airport.has_beacon,
      runways: vec![],
      frequencies: vec![],
      public: airport.public,
    }
  }
}

impl Airport {
  pub async fn select(icao: &str) -> Option<Self> {
    let pool = db::pool();

    let airport: Option<AirportRow> = sqlx::query_as(&format!(
      r#"
      SELECT * FROM {} WHERE icao = $1
      "#,
      TABLE_NAME
    ))
    .bind(icao)
    .fetch_optional(pool)
    .await
    .unwrap_or_else(|err| {
      log::error!("Unable to find airport '{}'", icao);
      None
    });

    match airport {
      Some(a) => Some(a.into()),
      None => None,
    }
  }

  pub async fn select_all() -> ApiResult<Vec<Self>> {
    let pool = db::pool();

    let airports: Vec<AirportRow> = sqlx::query_as(&format!(
      r#"
      SELECT * FROM {}
      "#,
      TABLE_NAME
    ))
    .fetch_all(pool)
    .await?;

    Ok(airports.into_iter().map(From::from).collect())
  }

  pub async fn insert(&self) -> ApiResult<Self> {
    let pool = db::pool();

    let airport: AirportRow = sqlx::query_as(&format!(
      r#"
      INSERT INTO {} (
        icao, iata, local, name, category, iso_country, iso_region, municipality,
        elevation_ft, longitude, latitude, has_tower, has_beacon, public
      )
      VALUES (
        $1, $2, $3, $4, $5, $6, $7,
        $8, $9, $10, $11, $12, $13, $14
      )
      RETURNING *
      "#,
      TABLE_NAME,
    ))
    .bind(self.icao.to_string())
    .bind(&self.iata)
    .bind(&self.local)
    .bind(self.name.to_string())
    .bind(self.category.to_string())
    .bind(self.iso_country.to_string())
    .bind(self.iso_region.to_string())
    .bind(self.municipality.to_string())
    .bind(self.elevation_ft)
    .bind(self.longitude)
    .bind(self.latitude)
    .bind(self.has_tower)
    .bind(self.has_beacon)
    .bind(self.public)
    .fetch_one(pool)
    .await?;

    Ok(airport.into())
  }

  pub async fn insert_all(airports: Vec<Self>) -> ApiResult<()> {
    let pool = db::pool();
    let airport_rows: Vec<AirportRow> = airports.into_iter().map(Into::into).collect();

    // Define the maximum size of a single insertion batch.
    let chunk_size = 1000;
    for chunk in airport_rows.chunks(chunk_size) {
      // Build a dynamic query for batch insertion.
      let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
        "INSERT INTO airports (icao, iata, local, name, category, \
        iso_country, iso_region, municipality, elevation_ft, \
        longitude, latitude, has_tower, has_beacon, public) ",
      );
      query_builder.push_values(chunk, |mut b, row| {
        b.push_bind(&row.icao)
          .push_bind(&row.iata)
          .push_bind(&row.local)
          .push_bind(&row.name)
          .push_bind(&row.category)
          .push_bind(&row.iso_country)
          .push_bind(&row.iso_region)
          .push_bind(&row.municipality)
          .push_bind(row.elevation_ft)
          .push_bind(row.longitude)
          .push_bind(row.latitude)
          .push_bind(row.has_tower)
          .push_bind(row.has_beacon)
          .push_bind(row.public);
      });

      let query = query_builder.build();
      query.execute(pool).await?;
    }

    Ok(())
  }

  // TODO
  pub async fn update(icao: &str, airport: &UpdateAirport) -> ApiResult<()> {
    Ok(())
  }

  pub async fn delete(icao: &str) -> ApiResult<()> {
    let pool = db::pool();

    sqlx::query(&format!(
      r#"
      DELETE FROM {} WHERE icao = $1
      "#,
      TABLE_NAME
    ))
    .bind(icao.to_string())
    .execute(pool)
    .await?;

    Ok(())
  }

  pub async fn delete_all() -> ApiResult<()> {
    let pool = db::pool();

    sqlx::query(&format!(
      r#"
      DELETE FROM {} WHERE true
      "#,
      TABLE_NAME
    ))
    .execute(pool)
    .await?;

    Ok(())
  }
}
