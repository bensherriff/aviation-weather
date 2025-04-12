use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, QueryBuilder};
use uuid::Uuid;
use crate::db;
use crate::error::ApiResult;

const TABLE_NAME: &str = "runways";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Runway {
  #[serde(rename = "id")]
  pub runway_id: String,
  pub length_ft: f32,
  pub width_ft: f32,
  pub surface: String,
}

#[derive(Debug, Deserialize, sqlx::FromRow)]
pub struct RunwayRow {
  pub id: Uuid,
  pub icao: String,
  pub runway_id: String,
  pub length_ft: f32,
  pub width_ft: f32,
  pub surface: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateRunway {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub icao: Option<String>,
  #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
  pub frequency_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub length_ft: Option<f32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub width_ft: Option<f32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub surface: Option<String>,
}

impl From<RunwayRow> for Runway {
  fn from(runway: RunwayRow) -> Self {
    Self {
      runway_id: runway.runway_id.clone(),
      length_ft: runway.length_ft.clone(),
      width_ft: runway.width_ft.clone(),
      surface: runway.surface.clone(),
    }
  }
}

impl Runway {
  pub fn into(runway: &Runway, icao: &str) -> RunwayRow {
    RunwayRow {
      id: Uuid::new_v4(),
      icao: icao.to_string(),
      runway_id: runway.runway_id.clone(),
      length_ft: runway.length_ft.clone(),
      width_ft: runway.width_ft.clone(),
      surface: runway.surface.clone(),
    }
  }

  pub async fn select_all_map(icaos: Vec<String>) -> ApiResult<HashMap<String, Vec<Self>>> {
    let pool = db::pool();

    let runway_rows: Vec<RunwayRow> = sqlx::query_as(&format!(
      r#"SELECT * FROM {} WHERE icao = ANY($1)"#,
      TABLE_NAME
    ))
    .bind(&icaos)
    .fetch_all(pool)
    .await?;

    let mut runway_map: HashMap<String, Vec<Self>> = HashMap::new();
    for runway_row in runway_rows {
      let icao = runway_row.icao.clone();
      let runway = runway_row.into();
      runway_map.entry(icao.to_string()).or_default().push(runway);
    }

    Ok(runway_map)
  }

  pub async fn select_all(icao: &str) -> ApiResult<Vec<Self>> {
    let pool = db::pool();

    let runway_rows: Vec<RunwayRow> = sqlx::query_as(&format!(
      r#"
      SELECT * FROM {} WHERE icao = $1
      "#,
      TABLE_NAME
    ))
    .bind(icao)
    .fetch_all(pool)
    .await?;
    Ok(runway_rows.into_iter().map(From::from).collect())
  }

  pub async fn insert_all(runways: &Vec<RunwayRow>) -> ApiResult<()> {
    let pool = db::pool();
    let chunk_size = 1000;

    for chunk in runways.chunks(chunk_size) {
      let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(&format!(
        "INSERT INTO {} (id, icao, runway_id, length_ft, width_ft, surface) ",
        TABLE_NAME
      ));
      query_builder.push_values(chunk, |mut b, row| {
        b.push_bind(&row.id)
          .push_bind(&row.icao)
          .push_bind(&row.runway_id)
          .push_bind(&row.length_ft)
          .push_bind(&row.width_ft)
          .push_bind(&row.surface);
      });

      let query = query_builder.build();
      query.execute(pool).await?;
    }

    Ok(())
  }
}
