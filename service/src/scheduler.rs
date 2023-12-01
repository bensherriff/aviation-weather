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
      let mut observation_time = chrono::Utc::now().timestamp();

      while peekable.peek().is_some() {
        let chunk: Vec<String> = peekable.by_ref().take(limit as usize).collect();
        let icao_string = chunk.join(",");
        trace!("Updating METARS for: {}", icao_string);
        match Metar::get_all(icao_string).await {
          Ok(metars) => {
            // Find the oldest observation time
            for metar in metars {
              if metar.observation_time.timestamp() < observation_time {
                observation_time = metar.observation_time.timestamp();
              }
            }
            sleep(Duration::from_millis(100)).await;
          },
          Err(err) => {
            warn!("{}", err);
          }
        }
      }
      debug!("METAR update complete");
      // Sleep until the earliest observation time is 1 hour old
      // Bounded by 1 and 3600 seconds
      let now = chrono::Utc::now().timestamp();
      let sleep_time = std::cmp::min(std::cmp::max(1, now - (observation_time + (3600))), 3600);
      debug!("Next update in {} seconds", sleep_time);
      sleep(Duration::from_secs(sleep_time as u64)).await;
    }
  });
}