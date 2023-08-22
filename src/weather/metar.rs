use regex::Regex;

use super::WeatherError;

#[derive(Debug)]
pub struct Metar {
    pub icao: String,
    pub date_time: String,
    pub report_modifier: String,
    pub wind_direction: String,
    pub wind_speed: String,
    pub wind_direction_variable: String,
    pub visibility: String,
    pub runway_visual_range: String,
    pub weather_phenomena: Vec<String>,
    pub sky_condition: Vec<String>,
    pub temperature: String,
    pub dew_point: String,
    pub altimeter: String,
    pub remarks: Vec<String>
}

impl Metar {
    pub fn new(input: String) -> Result<Metar, WeatherError> {
        if input.is_empty() {
            return Err(WeatherError("Input is empty".to_string()))
        }
        let mut offset: usize = 0;
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() < 5 {
            return Err(WeatherError("Unable to parse input".to_string()))
        }

        let mut report_modifier = "";
        if parts[2] == "AUTO" {
            offset += 1;
            report_modifier = parts[2];
        }

        let wind_re = Regex::new(r"^\d{3}V\d{3}$").unwrap();
        let wind_direction_variable = match wind_re.find(parts[3 + offset]) {
            Some(_) => {
                offset += 1;
                parts[3 + offset]
            },
            None => ""
        };

        let mut runway_visual_range = "";
        if parts[4 + offset].ends_with("FT") {
            offset += 1;
            runway_visual_range = parts[4 + offset];
        }

        let mut weather_phenomena: Vec<String> = vec![];
        let weather_re = Regex::new(r"^(-|\+)?[A-Z]{2}(?:[A-Z]{2})?$").unwrap();
        let mut sky_condition: Vec<String> = vec![];
        let sky_re = Regex::new(r"^CLR|(FEW|SCT|BKN|OVC)\d{3}$").unwrap();
        for n in (4 + offset)..parts.len() {
            match weather_re.find(parts[n]) {
                Some(_) => {
                    offset += 1;
                    weather_phenomena.push(parts[n].to_string());
                },
                None => {}
            }
            match sky_re.find(parts[n]) {
                Some(_) => {
                    offset += 1;
                    sky_condition.push(parts[n].to_string());
                },
                None => {}
            }
        }

        let temp_dew: Vec<&str> = parts[4 + offset].split("/").collect();
        let mut remarks: Vec<String> = vec![];
        if parts.len() > 6 + offset {
            // Skip the RMK string for remarks, starting at index + 1
            for n in (6 + offset + 1)..parts.len() {
                remarks.push(parts[n].to_string())
            }
        }

        Ok(Metar {
            icao: parts[0].to_string(),
            date_time: parts[1].to_string(),
            report_modifier: report_modifier.to_string(),
            wind_direction: parts[2 + offset][..3].to_string(),
            wind_speed: parts[2 + offset][3..].to_string(),
            wind_direction_variable: wind_direction_variable.to_string(),
            visibility: parts[3 + offset].to_string(),
            runway_visual_range: runway_visual_range.to_string(),
            weather_phenomena,
            sky_condition,
            temperature: temp_dew[0].to_string(),
            dew_point: temp_dew[1].to_string(),
            altimeter: parts[5 + offset].to_string(),
            remarks
        })
    }
}