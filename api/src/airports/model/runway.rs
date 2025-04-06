use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Runway {
  pub id: String,
  pub length_ft: f32,
  pub width_ft: f32,
  pub surface: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateRunway {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub length_ft: Option<f32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub width_ft: Option<f32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub surface: Option<String>,
}
