use std::collections::HashMap;
use std::str::FromStr;
use actix_web::web::Json;
use futures_util::try_join;
use moka::future::Cache;
use serde::{Deserialize, Serialize};
use sqlx::{Execute, Postgres, QueryBuilder};
use crate::airports::model::airport_category::AirportCategory;
use crate::airports::{Frequency, FrequencyRow, Runway, RunwayRow, UpdateFrequency, UpdateRunway};
use crate::db;
use crate::error::{ApiResult, Error};
use crate::metars::Metar;

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
  #[serde(skip_serializing_if = "Option::is_none")]
  pub latest_metar: Option<Metar>,
}

#[derive(Debug, Deserialize)]
pub struct AirportQuery {
  pub page: Option<u32>,
  pub limit: Option<u32>,
  pub icaos: Option<String>,
  pub iatas: Option<String>,
  pub locals: Option<String>,
  pub names: Option<String>,
  pub categories: Option<String>,
  pub iso_countries: Option<String>,
  pub iso_regions: Option<String>,
  pub municipalities: Option<String>,
  pub metars: Option<bool>,
}

impl Default for AirportQuery {
  fn default() -> Self {
    Self {
      page: Some(1),
      limit: Some(1000),
      icaos: None,
      iatas: None,
      locals: None,
      names: None,
      categories: None,
      iso_countries: None,
      iso_regions: None,
      municipalities: None,
      metars: None,
    }
  }
}

#[derive(Debug, Deserialize, sqlx::FromRow)]
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

#[derive(Debug, Deserialize)]
pub struct UpdateAirport {
  pub icao: Option<String>,
  pub iata: Option<String>,
  pub local: Option<String>,
  pub name: Option<String>,
  pub category: Option<AirportCategory>,
  pub iso_country: Option<String>,
  pub iso_region: Option<String>,
  pub municipality: Option<String>,
  pub elevation_ft: Option<f32>,
  pub longitude: Option<f32>,
  pub latitude: Option<f32>,
  pub has_tower: Option<bool>,
  pub has_beacon: Option<bool>,
  pub runways: Option<Vec<UpdateRunway>>,
  pub frequencies: Option<Vec<UpdateFrequency>>,
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
    Self {
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
      latest_metar: None,
    }
  }
}

impl Airport {
  pub async fn select(icao: &str, metar: bool) -> Option<Self> {
    let pool = db::pool();

    let airport_fut = async {
      sqlx::query_as(&format!("SELECT * FROM {} WHERE icao = $1", TABLE_NAME))
        .bind(icao)
        .fetch_optional(pool)
        .await
    };

    let metar_fut = async {
      if metar {
        match Metar::find_all(&vec![icao.to_string()]).await {
          Ok(m) => Some(m.into_iter().nth(0)),
          Err(err) => {
            log::error!("{}", err);
            None
          }
        }
      } else {
        None
      }
    };

    let runways_fut = Runway::select_all(icao);
    let frequencies_fut = Frequency::select_all(icao);

    let (airport_result, runways_result, frequencies_result, metar_result) =
      tokio::join!(airport_fut, runways_fut, frequencies_fut, metar_fut);

    let airport_row: Option<AirportRow> = match airport_result {
      Ok(opt) => opt,
      Err(err) => {
        log::error!("Unable to find airport '{}': {}", icao, err);
        return None;
      }
    };

    let runways: Vec<Runway> = match runways_result {
      Ok(r) => r,
      Err(err) => {
        log::error!("Error retrieving runways for airport '{}': {}", icao, err);
        vec![]
      }
    };

    let frequencies: Vec<Frequency> = match frequencies_result {
      Ok(f) => f,
      Err(err) => {
        log::error!(
          "Error retrieving frequencies for airport '{}': {}",
          icao,
          err
        );
        vec![]
      }
    };

    let metar: Option<Metar> = match metar_result {
      Some(m_option) => match m_option {
        Some(m) => Some(m),
        None => None,
      },
      None => None,
    };

    airport_row.map(|row| {
      let mut airport: Airport = row.into();
      airport.runways = runways;
      airport.frequencies = frequencies;
      airport.latest_metar = metar;
      airport
    })
  }

  pub async fn select_all(query: &AirportQuery) -> ApiResult<Vec<Self>> {
    let pool = db::pool();

    let mut builder = QueryBuilder::<Postgres>::new("SELECT * FROM ");
    builder.push(TABLE_NAME);

    let mut has_where = false;
    Self::push_condition_array(&mut builder, &mut has_where, "icao", &query.icaos);
    Self::push_condition_array(&mut builder, &mut has_where, "iata", &query.iatas);
    Self::push_condition_array(
      &mut builder,
      &mut has_where,
      "iso_country",
      &query.iso_countries,
    );
    Self::push_condition_array(
      &mut builder,
      &mut has_where,
      "iso_region",
      &query.iso_regions,
    );
    Self::push_condition_array(
      &mut builder,
      &mut has_where,
      "municipality",
      &query.municipalities,
    );
    Self::push_condition_array(&mut builder, &mut has_where, "local", &query.locals);
    Self::push_condition_array(&mut builder, &mut has_where, "name", &query.names);
    Self::push_condition_array(&mut builder, &mut has_where, "category", &query.categories);

    // Apply pagination.
    if let Some(limit) = query.limit {
      builder.push(" LIMIT ").push_bind(limit as i64);
      let offset = if let Some(page) = query.page {
        (page.saturating_sub(1) * limit) as i64
      } else {
        0
      };
      builder.push(" OFFSET ").push_bind(offset);
    }

    let airport_query = builder.build_query_as::<AirportRow>();
    let airport_rows: Vec<AirportRow> = airport_query.fetch_all(pool).await?;
    let mut airports: Vec<Airport> = airport_rows.into_iter().map(From::from).collect();

    if airports.is_empty() {
      return Ok(airports);
    }

    // Bulk update airport sub-fields
    let icaos: Vec<String> = airports.iter().map(|a| a.icao.clone()).collect();

    let runway_future = Runway::select_all_map(icaos.clone());
    let frequency_future = Frequency::select_all_map(icaos.clone());
    let metar_future = if query.metars.unwrap_or(false) {
      Some(Metar::find_all(&icaos))
    } else {
      None
    };

    let (runway_map, frequency_map, mut metars_opt) = match metar_future {
      Some(future_metars) => {
        let (runway_map, frequency_map, metars) =
          try_join!(runway_future, frequency_future, future_metars)?;
        (
          runway_map,
          frequency_map,
          Some(
            metars
              .into_iter()
              .map(|m| (m.station_id.clone(), m))
              .collect::<HashMap<_, _>>(),
          ),
        )
      }
      None => {
        let (runway_map, frequency_map) = try_join!(runway_future, frequency_future)?;
        (runway_map, frequency_map, None)
      }
    };

    for airport in airports.iter_mut() {
      airport.runways = runway_map.get(&airport.icao).cloned().unwrap_or_default();
      airport.frequencies = frequency_map
        .get(&airport.icao)
        .cloned()
        .unwrap_or_default();
      if let Some(ref mut metar_map) = metars_opt {
        airport.latest_metar = metar_map.remove(&airport.icao);
      }
    }

    Ok(airports)
  }

  pub async fn count(query: &AirportQuery) -> i64 {
    let pool = db::pool();

    let mut builder = QueryBuilder::<Postgres>::new("SELECT COUNT(*) FROM ");
    builder.push(TABLE_NAME);

    let mut has_where = false;
    Self::push_condition_array(&mut builder, &mut has_where, "icao", &query.icaos);
    Self::push_condition_array(&mut builder, &mut has_where, "iata", &query.iatas);
    Self::push_condition_array(
      &mut builder,
      &mut has_where,
      "iso_country",
      &query.iso_countries,
    );
    Self::push_condition_array(
      &mut builder,
      &mut has_where,
      "iso_region",
      &query.iso_regions,
    );
    Self::push_condition_array(
      &mut builder,
      &mut has_where,
      "municipality",
      &query.municipalities,
    );
    Self::push_condition_array(&mut builder, &mut has_where, "local", &query.locals);
    Self::push_condition_array(&mut builder, &mut has_where, "name", &query.names);
    Self::push_condition_array(&mut builder, &mut has_where, "category", &query.categories);

    let sql_query = builder.build_query_scalar();
    sql_query.fetch_one(pool).await.unwrap_or_else(|_| 0)
  }

  pub async fn insert(&self) -> ApiResult<Self> {
    let pool = db::pool();

    let mut all_runway_rows: Vec<RunwayRow> = Vec::new();
    let mut all_frequency_rows: Vec<FrequencyRow> = Vec::new();
    for runway in &self.runways {
      all_runway_rows.push(Runway::into(runway, &self.icao));
    }
    for frequency in &self.frequencies {
      all_frequency_rows.push(Frequency::into(frequency, &self.icao));
    }
    Runway::insert_all(&all_runway_rows).await?;
    Frequency::insert_all(&all_frequency_rows).await?;

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
    let chunk_size = 1000;
    let mut all_runway_rows: Vec<RunwayRow> = Vec::new();
    let mut all_frequency_rows: Vec<FrequencyRow> = Vec::new();
    let airport_rows: Vec<AirportRow> = airports
      .into_iter()
      .map(|airport| {
        for runway in &airport.runways {
          all_runway_rows.push(Runway::into(runway, &airport.icao));
        }
        for frequency in &airport.frequencies {
          all_frequency_rows.push(Frequency::into(frequency, &airport.icao));
        }
        airport.into()
      })
      .collect();
    Runway::insert_all(&all_runway_rows).await?;
    Frequency::insert_all(&all_frequency_rows).await?;

    for chunk in airport_rows.chunks(chunk_size) {
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

  fn push_condition_array<'a>(
    builder: &mut QueryBuilder<'a, Postgres>,
    has_where: &mut bool,
    column: &str,
    field: &'a Option<String>,
  ) {
    if let Some(ref value_str) = field {
      // Split on commas, trim whitespace, and drop empties.
      let values: Vec<&str> = value_str
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect();
      if !values.is_empty() {
        if !*has_where {
          builder.push(" WHERE ");
          *has_where = true;
        } else {
          builder.push(" AND ");
        }
        builder.push(column);
        builder.push(" = ANY(");
        builder.push_bind(values);
        builder.push(")");
      }
    }
  }
}
