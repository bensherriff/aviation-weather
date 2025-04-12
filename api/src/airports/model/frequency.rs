use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, QueryBuilder};
use uuid::Uuid;
use crate::db;
use crate::error::ApiResult;

const TABLE_NAME: &str = "frequencies";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Frequency {
  #[serde(rename = "id")]
  pub frequency_id: String,
  pub frequency_mhz: f32,
}

#[derive(Debug, Deserialize, sqlx::FromRow)]
pub struct FrequencyRow {
  pub id: Uuid,
  pub icao: String,
  pub frequency_id: String,
  pub frequency_mhz: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateFrequency {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub icao: Option<String>,
  #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
  pub frequency_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub frequency_mhz: Option<f32>,
}

impl From<FrequencyRow> for Frequency {
  fn from(frequency: FrequencyRow) -> Self {
    Self {
      frequency_id: frequency.frequency_id.clone(),
      frequency_mhz: frequency.frequency_mhz,
    }
  }
}

impl Frequency {
  pub fn into(frequency: &Frequency, icao: &str) -> FrequencyRow {
    FrequencyRow {
      id: Uuid::new_v4(),
      icao: icao.to_string(),
      frequency_id: frequency.frequency_id.clone(),
      frequency_mhz: frequency.frequency_mhz.clone(),
    }
  }

  pub async fn select_all_map(icaos: Vec<String>) -> ApiResult<HashMap<String, Vec<Self>>> {
    let pool = db::pool();

    let frequency_rows: Vec<FrequencyRow> = sqlx::query_as(&format!(
      r#"SELECT * FROM {} WHERE icao = ANY($1)"#,
      TABLE_NAME
    ))
    .bind(&icaos)
    .fetch_all(pool)
    .await?;

    let mut frequency_map: HashMap<String, Vec<Self>> = HashMap::new();
    for frequency_row in frequency_rows {
      let icao = frequency_row.icao.clone();
      let frequency = frequency_row.into();
      frequency_map
        .entry(icao.to_string())
        .or_default()
        .push(frequency);
    }

    Ok(frequency_map)
  }

  pub async fn select_all(icao: &str) -> ApiResult<Vec<Self>> {
    let pool = db::pool();

    let frequency_row: Vec<FrequencyRow> = sqlx::query_as(&format!(
      r#"
      SELECT * FROM {} WHERE icao = $1
      "#,
      TABLE_NAME
    ))
    .bind(icao)
    .fetch_all(pool)
    .await?;
    Ok(frequency_row.into_iter().map(From::from).collect())
  }

  pub async fn insert_all(frequencies: &Vec<FrequencyRow>) -> ApiResult<()> {
    let pool = db::pool();
    let chunk_size = 1000;

    for chunk in frequencies.chunks(chunk_size) {
      let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(&format!(
        "INSERT INTO {} (id, icao, frequency_id, frequency_mhz) ",
        TABLE_NAME
      ));
      query_builder.push_values(chunk, |mut b, row| {
        b.push_bind(&row.id)
          .push_bind(&row.icao)
          .push_bind(&row.frequency_id)
          .push_bind(&row.frequency_mhz);
      });

      let query = query_builder.build();
      query.execute(pool).await?;
    }

    Ok(())
  }
}
