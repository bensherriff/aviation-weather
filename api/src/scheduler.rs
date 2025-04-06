use tokio::time::{sleep, Duration};

// use crate::airports::{AirportDb, AirportFilter};
use crate::metars::Metar;

pub fn update_airports() {
  // tokio::spawn(async {
  //   let mut airports: Vec<AirportDb> = vec![];
  //   let limit = 100;
  //   loop {
  //     log::debug!("METAR update start");
  //     let total = match AirportDb::count(&AirportFilter::default()).await {
  //       Ok(t) => t,
  //       Err(err) => {
  //         log::warn!("{}", err);
  //         break;
  //       }
  //     };
  //     if total != airports.len() as i64 {
  //       log::debug!("{} cached airports, expected {}", airports.len(), total);
  //       airports = vec![];
  //       let pages = ((total as f32) / (if limit <= 0 { 1 } else { limit } as f32)).ceil() as i32;
  //       for page in 1..(pages + 1) {
  //         match AirportDb::find_all(&AirportFilter::default(), limit, page).await {
  //           Ok(mut a) => airports.append(&mut a),
  //           Err(err) => {
  //             log::warn!("{}", err);
  //             break;
  //           }
  //         }
  //       }
  //     }
  //     log::debug!("Updating {} airport METARS", airports.len());
  //
  //     let airport_icaos: Vec<String> = airports.iter().map(|a| a.icao.to_string()).collect();
  //     let mut peekable = airport_icaos.into_iter().peekable();
  //     let mut observation_time = chrono::Utc::now().timestamp();
  //
  //     if peekable.peek().is_none() {
  //       log::debug!("No airports to update, sleeping for 1 hour");
  //       sleep(Duration::from_secs(3600)).await;
  //       continue;
  //     }
  //
  //     while peekable.peek().is_some() {
  //       let chunk: Vec<String> = peekable.by_ref().take(limit as usize).collect();
  //       let icao_string = chunk.join(",");
  //       log::warn!("Updating METARS for: {}", &icao_string); // TODO: back to trace after
  //       match Metar::find_all(&[&icao_string]).await {
  //         Ok(metars) => {
  //           // Find the oldest observation time
  //           for metar in metars {
  //             if metar.observation_time.timestamp() < observation_time {
  //               observation_time = metar.observation_time.timestamp();
  //             }
  //           }
  //         }
  //         Err(err) => {
  //           log::warn!("{}", err);
  //         }
  //       }
  //       // Sleep for 100ms between chunks to avoid rate limiting
  //       sleep(Duration::from_millis(100)).await;
  //     }
  //     log::debug!("METAR update complete");
  //     // Sleep until the earliest observation time is 1 hour old
  //     // Bounded by 1 and 3600 seconds
  //     let now = chrono::Utc::now().timestamp();
  //     let sleep_time = std::cmp::min(std::cmp::max(1, now - (observation_time + 3600)), 3600);
  //     log::debug!("Next update in {} seconds", sleep_time);
  //     sleep(Duration::from_secs(sleep_time as u64)).await;
  //   }
  // });
}
