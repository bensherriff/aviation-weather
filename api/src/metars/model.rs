use crate::error::Error;
use crate::{error::ApiResult, db};
use chrono::{DateTime, Datelike, Utc};
use std::collections::HashSet;
use serde::{Deserialize, Serialize};

const TABLE_NAME: &str = "metars";

#[derive(Serialize, Deserialize, Debug)]
pub struct Metar {
  pub station_id: String, // icao
  pub raw_text: String,
  pub observation_time: DateTime<Utc>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub temp_c: Option<f64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub dewpoint_c: Option<f64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub wind_dir_degrees: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub wind_speed_kt: Option<f64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub wind_gust_kt: Option<f64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub variable_wind_dir_degrees: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub visibility_statute_mi: Option<String>,
  pub runway_visual_range: Vec<RunwayVisualRange>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub altim_in_hg: Option<f64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub sea_level_pressure_mb: Option<f64>,
  pub quality_control_flags: QualityControlFlags,
  pub weather_phenomena: Vec<String>,
  pub sky_condition: Vec<SkyCondition>,
  pub flight_category: FlightCategory,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub three_hr_pressure_tendency_mb: Option<f64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub max_t_c: Option<f64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub min_t_c: Option<f64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub precip_in: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RunwayVisualRange {
  pub runway: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub visibility_ft: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub variable_visibility_high_ft: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub variable_visibility_low_ft: Option<String>,
}

impl Default for RunwayVisualRange {
  fn default() -> Self {
    RunwayVisualRange {
      runway: "".to_string(),
      visibility_ft: None,
      variable_visibility_high_ft: None,
      variable_visibility_low_ft: None,
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QualityControlFlags {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub auto: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub auto_station_without_precipication: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub auto_station_with_precipication: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub maintenance_indicator_on: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub corrected: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub no_significant_change: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub temporary_change: Option<bool>,
}

impl Default for QualityControlFlags {
  fn default() -> Self {
    QualityControlFlags {
      auto: None,
      auto_station_without_precipication: None,
      auto_station_with_precipication: None,
      maintenance_indicator_on: None,
      corrected: None,
      no_significant_change: None,
      temporary_change: None,
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SkyCondition {
  pub sky_cover: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub cloud_base_ft_agl: Option<i32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub significant_convective_clouds: Option<String>,
}

impl Default for SkyCondition {
  fn default() -> Self {
    SkyCondition {
      sky_cover: "".to_string(),
      cloud_base_ft_agl: None,
      significant_convective_clouds: None,
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum FlightCategory {
  VFR,
  MVFR,
  LIFR,
  IFR,
  UNKN,
}

impl Default for Metar {
  fn default() -> Self {
    Self {
      raw_text: "".to_string(),
      station_id: "".to_string(),
      observation_time: chrono::DateTime::parse_from_rfc3339("1970-01-01T00:00:00Z")
        .unwrap()
        .with_timezone(&Utc),
      temp_c: None,
      dewpoint_c: None,
      wind_dir_degrees: None,
      wind_speed_kt: None,
      wind_gust_kt: None,
      variable_wind_dir_degrees: None,
      visibility_statute_mi: None,
      runway_visual_range: vec![],
      altim_in_hg: None,
      sea_level_pressure_mb: None,
      quality_control_flags: QualityControlFlags::default(),
      weather_phenomena: vec![],
      sky_condition: vec![],
      flight_category: FlightCategory::UNKN,
      three_hr_pressure_tendency_mb: None,
      max_t_c: None,
      min_t_c: None,
      precip_in: None,
    }
  }
}

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug)]
struct MetarDb {
  icao: String,
  observation_time: DateTime<Utc>,
  raw_text: String,
  data: serde_json::Value,
}

impl Metar {
  fn parse_multiple(metar_strings: &Vec<&str>) -> ApiResult<Vec<Self>> {
    let mut metars: Vec<Metar> = vec![];
    for metar_string in metar_strings {
      match Metar::parse(metar_string) {
        Ok(metar) => metars.push(metar),
        Err(e) => {
          log::warn!("Failed to parse metar string: {}", e);
          continue;
        }
      };
    }

    Ok(metars)
  }

  fn parse(metar_string: &str) -> ApiResult<Self> {
    if metar_string.is_empty() {
      return Err(Error::new(
        404,
        "Unable to parse empty METAR data".to_string(),
      ));
    }

    log::trace!("Parsing METAR data: {}", metar_string);
    let mut metar: Metar = Metar::default();
    metar.raw_text = metar_string.to_owned();
    let mut metar_parts: Vec<&str> = metar_string.split_whitespace().collect();
    if metar_parts.len() < 4 {
      return Err(Error::new(
        500,
        format!(
          "Unable to parse METAR data in an unexpected format: {}",
          metar_string
        ),
      ));
    }

    // Station Identifier
    metar.station_id = metar_parts[0].to_string();
    metar_parts.remove(0);

    // Date/Time
    let observation_time = metar_parts[0];
    metar_parts.remove(0);
    if observation_time.len() != 7 {
      return Err(Error::new(
        500,
        format!(
          "Unable to parse observation time in {}: {}",
          observation_time, metar_string
        ),
      ));
    }
    let observation_time_day = match observation_time[0..2].parse::<u32>() {
      Ok(day) => day,
      Err(err) => return Err(err.into()),
    };
    let mut observation_time_hour = match observation_time[2..4].parse::<u32>() {
      Ok(hour) => hour,
      Err(err) => return Err(err.into()),
    };
    let observation_time_minute = match observation_time[4..6].parse::<u32>() {
      Ok(minute) => minute,
      Err(err) => return Err(err.into()),
    };
    let current_time = Utc::now().naive_utc();

    // Check if the observation time is from the previous month
    let observation_time_month = if current_time.day() > observation_time_day {
      current_time.month() - 1
    } else {
      current_time.month()
    };
    // Check if the observation time is from the previous year
    let observation_time_year = if current_time.month() > observation_time_month {
      current_time.year() - 1
    } else {
      current_time.year()
    };
    let observation_time = format!(
      "{:04}-{:02}-{:02}T{:02}:{:02}:00Z",
      observation_time_year,
      observation_time_month,
      observation_time_day,
      observation_time_hour,
      observation_time_minute
    );
    metar.observation_time = match chrono::DateTime::parse_from_rfc3339(&observation_time) {
      Ok(datetime) => datetime.with_timezone(&Utc),
      Err(err) => return Err(err.into()),
    };

    loop {
      if metar_parts.is_empty() {
        break;
      }
      // Report Modifiers
      if !metar_parts.is_empty() && metar_parts[0] == "AUTO" {
        metar.quality_control_flags.auto = Some(true);
        metar_parts.remove(0);
      }
      if !metar_parts.is_empty() && metar_parts[0] == "COR" {
        metar.quality_control_flags.corrected = Some(true);
        metar_parts.remove(0);
      }
      if !metar_parts.is_empty() && metar_parts[0] == "NOSIG" {
        metar.quality_control_flags.no_significant_change = Some(true);
        metar_parts.remove(0);
      }

      // Wind Direction and Speed
      let wind_re = regex::Regex::new(r"^(?:[0-9]{3}|VRB)[0-9]{2}(?:KT|MPS)$").unwrap();
      let wind_gust_re =
        regex::Regex::new(r"^(?:[0-9]{3}|VRB)[0-9]{2}G[0-9]{2}(?:KT|MPS)$").unwrap();
      // Handle input error where there is a space between the numbers and units
      let mut value: Option<String> = None;
      if metar_parts.len() >= 2
        && metar_parts[0].len() == 5
        && (metar_parts[1] == "KT" || metar_parts[1] == "MPS")
      {
        value = Some(format!("{}{}", metar_parts[0], metar_parts[1]));
        metar_parts.remove(0);
        metar_parts.remove(0);
      } else if metar_parts.len() >= 2
        && metar_parts[0].len() == 7
        && metar_parts[0].contains("G")
        && (metar_parts[1] == "KT" || metar_parts[1] == "MPS")
      {
        value = Some(format!("{}{}", metar_parts[0], metar_parts[1]));
        metar_parts.remove(0);
        metar_parts.remove(0);
      } else if !metar_parts.is_empty() && wind_re.is_match(metar_parts[0]) {
        value = Some(metar_parts[0].to_string());
        metar_parts.remove(0);
      } else if !metar_parts.is_empty() && wind_gust_re.is_match(metar_parts[0]) {
        value = Some(metar_parts[0].to_string());
        metar_parts.remove(0);
      }

      match value {
        Some(wind) => {
          if wind_re.is_match(&wind) {
            let wind_dir_degrees = &wind[0..3];
            metar.wind_dir_degrees = Some(wind_dir_degrees.to_string());
            let mut wind_speed_kt = wind[3..5].to_string();
            // Convert m/s to kt
            if wind.len() == 8 {
              wind_speed_kt = (wind_speed_kt.parse::<f64>().unwrap() * 1.94384).to_string();
            }
            metar.wind_speed_kt = Some(wind_speed_kt.parse::<f64>().unwrap());
          } else if wind_gust_re.is_match(&wind) {
            let wind_dir_degrees = &wind[0..3];
            metar.wind_dir_degrees = Some(wind_dir_degrees.to_string());
            let mut wind_speed_kt = wind[3..5].to_string();
            let mut wind_gust_kt = wind[6..8].to_string();
            // Convert m/s to kt
            if wind.len() == 9 {
              wind_speed_kt = (wind_speed_kt.parse::<f64>().unwrap() * 1.94384).to_string();
              wind_gust_kt = (wind_gust_kt.parse::<f64>().unwrap() * 1.94384).to_string();
            }
            metar.wind_speed_kt = Some(wind_speed_kt.parse::<f64>().unwrap());
            metar.wind_gust_kt = Some(wind_gust_kt.parse::<f64>().unwrap());
          }
        }
        None => {}
      }

      // Variable Wind Direction
      let variable_wind_re = regex::Regex::new(r"^[0-9]{3}V[0-9]{3}$").unwrap();
      if !metar_parts.is_empty() && variable_wind_re.is_match(metar_parts[0]) {
        metar.variable_wind_dir_degrees = Some(metar_parts[0].to_string());
        metar_parts.remove(0);
      }

      // Visibility
      let visibility_re = regex::Regex::new(r"^M?(?:[0-9]+|[0-9]+/[0-9]+)SM$").unwrap();
      let visibility_re_m = regex::Regex::new(r"^[0-9]{4}(:?N|NE|NW|S|SE|SW)?$").unwrap();
      if !metar_parts.is_empty() && visibility_re.is_match(metar_parts[0]) {
        let visibility_str = &metar_parts[0][0..metar_parts[0].len() - 2];
        metar_parts.remove(0);
        let visibility: String = if visibility_str.contains("/") {
          let visibility_parts: Vec<&str> = visibility_str.split("/").collect();
          let visibility_left = visibility_parts[0];
          let visibility_right = visibility_parts[1].parse::<f64>().unwrap();
          if visibility_left.starts_with("M") {
            format!(
              "M{}",
              visibility_left[1..visibility_left.len()]
                .parse::<f64>()
                .unwrap()
                / visibility_right
            )
          } else if visibility_left.starts_with("P") {
            format!(
              "P{}",
              visibility_left[1..visibility_left.len()]
                .parse::<f64>()
                .unwrap()
                / visibility_right
            )
          } else {
            format!(
              "{}",
              visibility_left.parse::<f64>().unwrap() / visibility_right
            )
          }
        } else {
          visibility_str.to_string()
        };
        metar.visibility_statute_mi = Some(visibility);
      } else if !metar_parts.is_empty()
        && metar_parts[0].parse::<f64>().is_ok()
        && metar_parts.len() > 1
        && visibility_re.is_match(metar_parts[1])
      {
        let visibility_whole = metar_parts[0].parse::<f64>().unwrap();
        metar_parts.remove(0);
        let visibility_parts: Vec<&str> = metar_parts[0].split("/").collect();
        metar_parts.remove(0);
        let visibility_left = visibility_parts[0];
        let visibility_right = visibility_parts[1][0..visibility_parts[1].len() - 2]
          .parse::<f64>()
          .unwrap();
        let visibility = if visibility_left.starts_with("M") {
          format!(
            "M{}",
            visibility_whole
              + (visibility_left[1..visibility_left.len()]
                .parse::<f64>()
                .unwrap()
                / visibility_right)
          )
        } else if visibility_left.starts_with("P") {
          format!(
            "P{}",
            visibility_whole
              + (visibility_left[1..visibility_left.len()]
                .parse::<f64>()
                .unwrap()
                / visibility_right)
          )
        } else {
          format!(
            "{}",
            visibility_whole + (visibility_left.parse::<f64>().unwrap() / visibility_right)
          )
        };
        metar.visibility_statute_mi = Some(visibility);
      } else if !metar_parts.is_empty() && visibility_re_m.is_match(metar_parts[0]) {
        // Convert meters to statute miles
        let visibility = metar_parts[0];
        metar_parts.remove(0);
        if &visibility[0..4] == "9999" {
          metar.visibility_statute_mi = Some("P10".to_string());
        } else {
          let visibility = visibility[0..4].parse::<f64>().unwrap() * 0.000621371;
          metar.visibility_statute_mi = Some(format!("{:.2}", visibility));
        }
      }

      // Runway Visual Range
      let rvr_re = regex::Regex::new(r"^R[0-9]{1,3}(?:L|R|C)?/[PM]?[0-9]{4}FT$").unwrap();
      let variable_rvr_re =
        regex::Regex::new(r"^R[0-9]{1,3}(?:L|R|C)?/[PM]?[0-9]{4}V[PM]?[0-9]{4}FT$").unwrap();
      while !metar_parts.is_empty()
        && (rvr_re.is_match(metar_parts[0]) || variable_rvr_re.is_match(metar_parts[0]))
      {
        let rvr_string = metar_parts[0];
        metar_parts.remove(0);
        let mut rvr = RunwayVisualRange::default();
        let rvr_parts: Vec<&str> = rvr_string.split("/").collect();
        rvr.runway = rvr_parts[0].to_string();
        if rvr_re.is_match(rvr_string) {
          rvr.visibility_ft = Some(rvr_parts[1].to_string());
        } else {
          let rvr_variable_parts: Vec<&str> = rvr_parts[1].split("V").collect();
          if rvr_variable_parts.len() != 2 {
            log::warn!(
              "Unable to parse runway visual range in {}: {}",
              rvr_string,
              metar_string
            );
          } else {
            rvr.variable_visibility_low_ft = Some(rvr_variable_parts[0].to_string());
            rvr.variable_visibility_high_ft = Some(rvr_variable_parts[1].to_string());
          }
        }
      }

      // Weather Phenomena
      let wx_intensity = "(?:[+-]|VC)?";
      let wx_descriptor = "(?:MI|PR|BC|DR|BL|SH|TS|FZ)?";
      let wx_precipitation =
        "(?:DZ|RA|SN|SG|IC|PL|GR|GS|UP|BR|FG|FU|VA|DU|SA|HZ|PY|PO|SQ|FC|SS|DS)?";
      let wx_re = regex::Regex::new(&format!(
        r"^{}{}{}$",
        wx_intensity, wx_descriptor, wx_precipitation
      ))
      .unwrap();
      while !metar_parts.is_empty() && wx_re.is_match(metar_parts[0]) {
        metar.weather_phenomena.push(metar_parts[0].to_string());
        metar_parts.remove(0);
      }

      // Sky Condition
      if !metar_parts.is_empty() && metar_parts[0] == "CAVOK" {
        metar.sky_condition.push(SkyCondition {
          sky_cover: "CLR".to_string(),
          cloud_base_ft_agl: None,
          significant_convective_clouds: None,
        });
        metar_parts.remove(0);
      }
      let sky_condition_re =
        regex::Regex::new(r"^(?:CLR|SKC|NSC|NCD|(?:FEW|SCT|BKN|OVC|VV)([0-9/]{3})?(?:CB|TCU)?)$")
          .unwrap();
      while !metar_parts.is_empty() && sky_condition_re.is_match(metar_parts[0]) {
        let sky_condition_string = metar_parts[0];
        metar_parts.remove(0);
        let mut sky_condition = SkyCondition::default();
        let mut vv_offset = 0;
        if &sky_condition_string[0..2] == "VV" {
          sky_condition.sky_cover = "VV".to_string();
          vv_offset = 1;
        } else {
          sky_condition.sky_cover = sky_condition_string[0..3].to_string();
        }
        if sky_condition_string.len() > 3 - vv_offset {
          // Parse out the next three digits
          let cloud_base_ft_agl = &sky_condition_string[3 - vv_offset..6 - vv_offset];
          if cloud_base_ft_agl == "///" {
            sky_condition.cloud_base_ft_agl = None;
          } else {
            sky_condition.cloud_base_ft_agl = match cloud_base_ft_agl.parse::<i32>() {
              Ok(c) => Some(c * 100),
              Err(err) => {
                log::warn!(
                  "Unable to parse cloud base in {}: {}",
                  sky_condition_string,
                  err
                );
                None
              }
            };
          }
          if sky_condition_string.len() > 6 - vv_offset {
            // Parse out the next two digits
            let scc = &sky_condition_string[6 - vv_offset..8 - vv_offset];
            sky_condition.significant_convective_clouds = Some(scc.to_string());
          }
        }
        metar.sky_condition.push(sky_condition);
      }

      // Temperature and Dewpoint
      let temp_re = regex::Regex::new(r"^(?:M?[0-9]{2})?/(?:M?[0-9]{2})?$").unwrap();
      if !metar_parts.is_empty() && temp_re.is_match(metar_parts[0]) {
        let temp_string = metar_parts[0];
        metar_parts.remove(0);
        let temp_parts: Vec<&str> = temp_string.split("/").collect();
        let mut temp_c = "";
        let mut dewpoint_c = "";
        if temp_parts.len() != 2 {
          if temp_string.ends_with("/") {
            temp_c = temp_parts[0];
          } else {
            dewpoint_c = temp_parts[0];
          }
        } else {
          temp_c = temp_parts[0];
          dewpoint_c = temp_parts[1];
        }
        if temp_c.starts_with("M") {
          metar.temp_c = Some(temp_c[1..temp_c.len()].parse::<f64>().unwrap() * -1.0);
        } else if !temp_c.is_empty() {
          metar.temp_c = match temp_c.parse::<f64>() {
            Ok(t) => Some(t),
            Err(err) => {
              log::warn!("Unable to parse temperature in {}: {}", temp_c, err);
              None
            }
          };
        }
        if dewpoint_c.starts_with("M") {
          metar.dewpoint_c = Some(dewpoint_c[1..dewpoint_c.len()].parse::<f64>().unwrap() * -1.0);
        } else if !dewpoint_c.is_empty() {
          metar.dewpoint_c = match dewpoint_c.parse::<f64>() {
            Ok(d) => Some(d),
            Err(err) => {
              log::warn!("Unable to parse dewpoint in {}: {}", dewpoint_c, err);
              None
            }
          };
        }
      }

      // Altimeter
      let altim_re = regex::Regex::new(r"^A[0-9]{4}$").unwrap();
      if !metar_parts.is_empty() && altim_re.is_match(metar_parts[0]) {
        let altim = metar_parts[0];
        metar_parts.remove(0);
        metar.altim_in_hg = Some(altim[1..altim.len()].parse::<f64>().unwrap() / 100.0);
      }

      // Pressure
      let pressure_re = regex::Regex::new(r"^Q[0-9]{4}$").unwrap();
      if !metar_parts.is_empty() && pressure_re.is_match(metar_parts[0]) {
        let pressure = metar_parts[0];
        metar_parts.remove(0);
        metar.sea_level_pressure_mb = Some(pressure[1..pressure.len()].parse::<f64>().unwrap());
      }

      // Temporary Change
      if !metar_parts.is_empty() && metar_parts[0] == "TEMPO" {
        metar.quality_control_flags.temporary_change = Some(true);
        metar_parts.remove(0);
      }

      // Remarks
      if !metar_parts.is_empty() && metar_parts[0] == "RMK" {
        metar_parts.remove(0);
        loop {
          if metar_parts.is_empty() {
            break;
          }
          let slp_re = regex::Regex::new(r"^SLP([0-9]{3})$").unwrap();
          let hourly_temp_re = regex::Regex::new(r"^T[01][0-9]{3}[01][0-9]{3}$").unwrap();
          let remark = metar_parts[0];
          metar_parts.remove(0);
          if remark == "AO1" {
            metar
              .quality_control_flags
              .auto_station_without_precipication = Some(true);
          } else if remark == "AO2" {
            metar.quality_control_flags.auto_station_with_precipication = Some(true);
          } else if remark == "$" {
            metar.quality_control_flags.maintenance_indicator_on = Some(true);
          } else if slp_re.is_match(remark) {
            let slp = slp_re.captures(remark).unwrap();
            let sea_level_pressure = slp[1].parse::<f64>().unwrap();
            if sea_level_pressure > 500.0 {
              metar.sea_level_pressure_mb = Some((sea_level_pressure / 10.0) + 900.0);
            } else {
              metar.sea_level_pressure_mb = Some((sea_level_pressure / 10.0) + 1000.0);
            }
          } else if hourly_temp_re.is_match(remark) {
            let temp_negation = &remark[1..2];
            let temp = &remark[2..5];
            if let Ok(t) = temp.parse::<f64>() {
              if temp_negation == "0" {
                metar.temp_c = Some(t / 10.0);
              } else {
                metar.temp_c = Some(t / 10.0 * -1.0);
              }
            }
            let dewpoint_negation = &remark[6..7];
            let dewpoint = &remark[6..9];
            if let Ok(d) = dewpoint.parse::<f64>() {
              if dewpoint_negation == "0" {
                metar.dewpoint_c = Some(d / 10.0);
              } else {
                metar.dewpoint_c = Some(d / 10.0 * -1.0);
              }
            }
          }
        }
      }

      // Skip unexpected fields
      if !metar_parts.is_empty() {
        log::warn!(
          "Skipping unexpected field: '{}' ({})",
          metar_parts[0],
          metar_string
        );
        metar_parts.remove(0);
      }
    }

    // Flight Category
    if metar.visibility_statute_mi.is_none() && metar.sky_condition.is_empty() {
      metar.flight_category = FlightCategory::UNKN;
    } else {
      let visibility = match &metar.visibility_statute_mi {
        Some(v) => {
          if v.starts_with("M") || v.starts_with("P") {
            v[1..v.len()].parse::<f64>().unwrap()
          } else {
            v.parse::<f64>().unwrap()
          }
        }
        None => 5.0, // Assume VFR if no visibility is present
      };
      // Ceiling is the lowest cloud base that is BKN or OVC
      let ceiling = match metar.sky_condition.first() {
        Some(s) => {
          if s.sky_cover == "VV" {
            0.0
          } else if s.sky_cover == "BKN" || s.sky_cover == "OVC" {
            match s.cloud_base_ft_agl {
              Some(c) => c as f64,
              None => 0.0,
            }
          } else {
            3000.0 // Assume VFR if no BKN or OVC sky condition is present
          }
        }
        None => 3000.0, // Assume VFR if no sky condition is present
      };
      if visibility >= 5.0 && ceiling >= 3000.0 {
        metar.flight_category = FlightCategory::VFR;
      } else if visibility >= 3.0 && ceiling >= 1000.0 {
        metar.flight_category = FlightCategory::MVFR;
      } else if visibility >= 1.0 && ceiling >= 500.0 {
        metar.flight_category = FlightCategory::IFR;
      } else {
        metar.flight_category = FlightCategory::LIFR;
      }
    }

    Ok(metar)
  }

  fn get_missing_metar_icaos(db_metars: &Vec<Self>, station_icaos: &[&str]) -> Vec<String> {
    let mut missing_metar_icaos: Vec<String> = vec![];
    let current_time = chrono::Local::now().naive_local().and_utc().timestamp();
    let db_metars_set: HashSet<&str> = db_metars
      .iter()
      .map(|icao| icao.station_id.as_str())
      .collect();
    let station_icaos_set: HashSet<&str> = station_icaos.to_owned().into_iter().collect();
    for difference in db_metars_set.symmetric_difference(&station_icaos_set) {
      missing_metar_icaos.push(difference.to_string());
    }
    for metar in db_metars {
      if current_time > (metar.observation_time.timestamp() + 3600) {
        log::trace!("{} METAR data is outdated", metar.station_id);
        missing_metar_icaos.push(metar.station_id.to_string());
      }
    }
    missing_metar_icaos
  }

  async fn get_remote_metars(icaos: &[&str]) -> ApiResult<Vec<Metar>> {
    let gov_api_url = std::env::var("GOV_API_URL").expect("GOV_API_URL must be set");
    // Query the remote API for the missing METAR data 10 at a time
    let icao_chunks = icaos
      .chunks(10)
      .map(|chunk| chunk.join(","))
      .collect::<Vec<String>>();
    let mut metars: Vec<Metar> = vec![];
    for icao_chunk in icao_chunks {
      let url = format!("{}/metar.php?ids={}", gov_api_url, icao_chunk);
      let mut m = match reqwest::get(url).await {
        Ok(r) => {
          // Check if the status code is 200
          if r.status() != 200 {
            return Err(Error::new(
              500,
              format!("Unable to get METAR request: {}", r.status()),
            ));
          }
          match r.text().await {
            Ok(r) => {
              let metar_chunk = r
                .trim()
                .split("\n")
                .filter(|m| !m.trim().is_empty())
                .collect();
              match Self::parse_multiple(&metar_chunk) {
                Ok(m) => m,
                Err(err) => return Err(err),
              }
            }
            Err(err) => {
              return Err(Error::new(
                500,
                format!("Unable to parse METAR request: {}", err),
              ))
            }
          }
        }
        Err(err) => {
          return Err(Error::new(
            500,
            format!("Unable to get METAR request: {}", err),
          ))
        }
      };
      metars.append(&mut m);
    }
    Ok(metars)
  }

  fn from_db(metar_db: MetarDb) -> ApiResult<Metar> {
    let metar: Metar = serde_json::from_value(metar_db.data)?;
    Ok(metar)
  }

  fn to_db(&self) -> ApiResult<MetarDb> {
    let data = serde_json::to_value(self)?;
    Ok(MetarDb {
      icao: self.station_id.clone(),
      observation_time: self.observation_time,
      raw_text: self.raw_text.clone(),
      data,
    })
  }

  pub async fn find_all(icao_list: &[&str]) -> ApiResult<Vec<Self>> {
    if icao_list.is_empty() {
      return Ok(Vec::new());
    }

    let pool = db::pool();
    let metar_dbs: Vec<MetarDb> = match sqlx::query_as::<_, MetarDb>(&format!(
      r#"
      SELECT DISTINCT ON (icao) * FROM {} WHERE icao = ANY($1) ORDER BY icao, observation_time DESC
      "#,
      TABLE_NAME
    ))
    .bind(icao_list)
    .fetch_all(pool)
    .await
    {
      Ok(m) => m,
      Err(err) => {
        return Err(Error::new(
          500,
          format!("Unable to find METARs with input {:?}: {}", icao_list, err),
        ));
      }
    };
    let mut metars: Vec<Metar> = metar_dbs
      .into_iter()
      .filter_map(|metar_db| Metar::from_db(metar_db).ok())
      .collect();

    // Check for missing metars
    let missing_icao_list = Self::get_missing_metar_icaos(&metars, icao_list);
    if !missing_icao_list.is_empty() {
      log::trace!("Retrieving missing METAR data for {:?}", missing_icao_list);

      let missing_icao_list: Vec<&str> = missing_icao_list.iter().map(|s| s.as_str()).collect();
      let mut missing_icao_list = Self::get_remote_metars(&missing_icao_list)
        .await
        .unwrap_or_else(|err| {
          log::warn!("Unable to get remote METAR data; {}", err);
          vec![]
        });

      if missing_icao_list.len() > 0 {
        // Insert missing METARs
        for missing_metar in &missing_icao_list {
          missing_metar.insert().await?;
        }
        metars.append(&mut missing_icao_list)
      }
    }

    Ok(metars)
  }

  pub async fn insert(&self) -> ApiResult<()> {
    let pool = db::pool();
    let metar: MetarDb = self.to_db()?;
    sqlx::query(&format!(
      r#"
      INSERT INTO {} (
        icao,
        observation_time,
        raw_text,
        data
      )
      VALUES ($1, $2, $3, $4)
      "#,
      TABLE_NAME,
    ))
    .bind(metar.icao)
    .bind(metar.observation_time)
    .bind(metar.raw_text)
    .bind(metar.data)
    .execute(pool)
    .await?;

    Ok(())
  }
}
