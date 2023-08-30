use std::error::Error;
use std::fmt;
use log::warn;
use std::io::BufRead;
use quick_xml::{Reader, events::{Event, BytesStart}, Writer, de::Deserializer};
use serde::Deserialize;

pub struct Airport {
    pub name: String,
    pub icao: String
}

impl Airport {
    pub fn new(name: String, icao: String) -> Airport {
        Airport { name, icao }
    }
}

#[derive(Debug)]
pub struct WeatherError(pub String);

impl fmt::Display for WeatherError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for WeatherError {}

#[derive(Deserialize, Debug)]
pub struct Metar {
    pub raw_text: String,
    pub station_id: String,
    pub observation_time: String,
    pub latitude: f32,
    pub longitude: f32,
    pub temp_c: f32,
    pub dewpoint_c: f32,
    pub wind_dir_degrees: i32,
    pub wind_speed_kt: i32,
    pub visibility_statute_mi: String,
    pub altim_in_hg: f32,
    pub sea_level_pressure_mb: Option<f32>,
    pub quality_control_flags: Option<QualityControlFlags>,
    pub wx_string: Option<String>,
    // pub sky_con dition: Option<Vec<String>>, // TODO work on attributes
    pub flight_category: String,
    pub three_hr_pressure_tendency_mb: Option<f32>,
    pub metar_type: String,
    #[serde(rename = "maxT_c")]
    pub max_t_c: Option<f32>,
    #[serde(rename = "minT_c")]
    pub min_t_c: Option<f32>,
    pub precip_in: Option<f32>,
    pub elevation_m: i32
}

#[derive(Deserialize, Debug)]
pub struct QualityControlFlags {
    pub auto: Option<bool>,
    pub auto_station: Option<bool>
}

impl Metar {
    pub fn parse(input: String) -> Result<Vec<Metar>, WeatherError> {
        if input.is_empty() {
            return Err(WeatherError("Input is empty".to_string()))
        }

        let mut reader = Reader::from_str(&input);
        let mut buf = Vec::new();
        let mut junk_buf: Vec<u8> = Vec::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Err(e) => panic!("Error at position: {}: {:?}", reader.buffer_position(), e),
                Ok(Event::Eof) => break,
                Ok(Event::Start(e)) => {
                    match e.name().as_ref() {
                        b"METAR" => {
                            let metar_bytes = Metar::read_to_end_into_buffer(&mut reader, &e, &mut junk_buf).unwrap();
                            let str = std::str::from_utf8(&metar_bytes).unwrap();
                            let mut deserializer = Deserializer::from_str(str);
                            let metar = Metar::deserialize(&mut deserializer).unwrap();
                            println!("{:#?}", metar);
                        },
                        _ => ()
                    }
                },
                _ => ()
            }
        }

        return Ok(vec![])
    }

    // https://capnfabs.net/posts/parsing-huge-xml-quickxml-rust-serde/
    pub fn read_to_end_into_buffer<R: BufRead>(reader: &mut Reader<R>, start_tag: &BytesStart, junk_buf: &mut Vec<u8>) -> Result<Vec<u8>, quick_xml::Error> {
        let mut depth = 0;
        let mut output_buf: Vec<u8> = Vec::new();
        let mut w = Writer::new(&mut output_buf);
        let tag_name = start_tag.name();
        w.write_event(Event::Start(start_tag.clone()))?;
        loop {
            junk_buf.clear();
            let event = reader.read_event_into(junk_buf)?;
            w.write_event(&event)?;
      
            match event {
                Event::Start(e) if e.name() == tag_name => depth += 1,
                Event::End(e) if e.name() == tag_name => {
                    if depth == 0 {
                        return Ok(output_buf);
                    }
                    depth -= 1;
                }
                Event::Eof => {
                    panic!("EOF")
                }
                _ => {}
            }
        }
    }
}

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
        let url = format!("{}/metar.php?ids={}&format=xml", self.base_url, station_string);
        
        let metars: Vec<Metar> = match reqwest::get(url).await {
            Ok(r) => match r.text().await {
                Ok(r) => {
                    match Metar::parse(r) {
                        Ok(m) => m,
                        Err(err) => {
                            warn!("{}", err);
                            vec![]
                        }
                    }
                },
                Err(err) => {
                    warn!("Unable to parse METAR request: {}", err);
                    vec![]
                }
            },
            Err(err) => {
                warn!("Unable to get METAR request: {}", err);
                vec![]
            }
        };
        return metars;
    }
}