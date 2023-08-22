use std::error::Error;

use airport::Airport;
use log::debug;
use crate::weather::{Weather, metar::Metar};

mod airport;
mod weather;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let mut weather = Weather {
        base_url: "https://beta.aviationweather.gov/cgi-bin/data".to_string()
    };
    let airports: Vec<Airport> = vec![
        Airport::new("Leesburg Executive Airport".to_string(), "KJYO".to_string()),
        Airport::new("Manassas Regional Airpoirt".to_string(), "KHEF".to_string()),
        Airport::new("Dulles International Airport".to_string(), "KIAD".to_string()),
        Airport::new("Frederick Municipal Airport".to_string(), "KFDK".to_string()),
        Airport::new("Eastern West Virginia Regional Airport".to_string(), "KMRB".to_string()),
        Airport::new("Winchester Regional Airport".to_string(), "KOKV".to_string()),
        Airport::new("Front Royal-Warren County Airport".to_string(), "KFRR".to_string()),
        Airport::new("Luray Caverns Airport".to_string(), "KLUA".to_string()),
        Airport::new("Shenandoah Valley Airport".to_string(), "KSHD".to_string()),
        Airport::new("Charlottesville-Albemarle Airport".to_string(), "KCHO".to_string()),
        Airport::new("Culpeper Regional Airport".to_string(), "KCJR".to_string()),
        Airport::new("Warrenton-Fauquier Airport".to_string(), "KHWY".to_string()),
        Airport::new("Stafford Regional Airport".to_string(), "KRMN".to_string()),
        Airport::new("Shannon Airport".to_string(), "KEZF".to_string()),
    ];

    let metars: Vec<Metar> = weather.metar(airports).await;
    debug!("{:#?}", metars);
    Ok(())
}
