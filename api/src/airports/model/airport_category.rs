use std::fmt::Display;
use std::str::FromStr;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
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
  #[serde(rename = "balloon_port")]
  BalloonPort,
  #[serde(rename = "unknown")]
  Unknown,
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
      "balloon_port" => Ok(AirportCategory::BalloonPort),
      _ => Ok(AirportCategory::Unknown),
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
      AirportCategory::BalloonPort => write!(f, "balloon_port"),
      AirportCategory::Unknown => write!(f, "unknown"),
    }
  }
}
