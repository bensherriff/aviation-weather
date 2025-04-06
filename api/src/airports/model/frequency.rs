use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Frequency {
  pub id: String,
  pub frequency_mhz: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateFrequency {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub frequency_mhz: Option<f32>,
}
