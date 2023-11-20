use tokio::time::{sleep, Duration};
use log::{warn, debug, trace};

use crate::airports::{QueryAirport, QueryFilters};
use crate::metars::Metar;

pub fn update_airports() {
  tokio::spawn(async {
    loop {
      debug!("METAR update start");
      let total = match QueryAirport::get_count(&QueryFilters::default()) {
        Ok(t) => t,
        Err(err) => {
          warn!("{}", err);
          break
        }
      };
      let limit = 50;
      let pages = ((total as f32) / (if limit <= 0 { 1 } else { limit} as f32)).ceil() as i32;
      let mut airports: Vec<QueryAirport> = vec![];
      for page in 1..(pages + 1) {
        match QueryAirport::get_all(&QueryFilters::default(), limit, page) {
          Ok(mut a) => {
            airports.append(&mut a)
          },
          Err(err) => {
            warn!("{}", err);
            break
          }
        }
      }
      debug!("Updating {} airport METARS", airports.len());

      let airport_icaos: Vec<String> = airports.iter().map(|a| a.icao.to_string()).collect();
      let mut peekable = airport_icaos.into_iter().peekable();
      while peekable.peek().is_some() {
        let chunk: Vec<String> = peekable.by_ref().take(limit as usize).collect();
        let icao_string = chunk.join(",");
        trace!("Updating METARS for: {}", icao_string);
        match Metar::get_all(icao_string).await {
          Ok(_) => {
            sleep(Duration::from_millis(100)).await;
          },
          Err(err) => {
            warn!("{}", err);
          }
        }
      }
      debug!("METAR update complete");
      sleep(Duration::from_secs(60 * 60)).await;
    }
  });
}