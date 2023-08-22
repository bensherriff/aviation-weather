use std::error::Error;
use std::fmt;
use log::warn;

use crate::airport::Airport;
use self::metar::Metar;

pub mod metar;

#[derive(Debug)]
pub struct WeatherError(pub String);

impl fmt::Display for WeatherError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for WeatherError {}

pub struct Weather {
    pub base_url: String
}

impl Weather {
    pub async fn metar(&mut self, airports: Vec<Airport>) -> Vec<Metar> {
        let mut station_icaos: Vec<&str> = vec![];
        for station in airports.iter() {
            station_icaos.push(&station.icao);
        }
        let station_string = station_icaos.join(",");
        let url = format!("{}/metar.php?ids={}", self.base_url, station_string);
        
        let mut metars: Vec<Metar> = vec![];
        match reqwest::get(url).await {
            Ok(r) => match r.text().await {
                Ok(r) => {
                    let lines: Vec<&str> = r.split("\n").collect();
                    for line in lines.iter() {
                        match Metar::new(line.to_string()) {
                            Ok(m) => metars.push(m),
                            Err(err) => warn!("{}", err)
                        };
                    }
                },
                Err(err) => warn!("Unable to parse METAR request: {}", err)
            },
            Err(err) => warn!("Unable to get METAR request: {}", err)
        }
        return metars;
    }
}