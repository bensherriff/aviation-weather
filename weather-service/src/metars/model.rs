use crate::{error_handler::CustomError, db};
use crate::schema::metars;
use diesel::prelude::*;
use log::{warn, error, debug};
use std::io::BufRead;
use quick_xml::{Reader, events::{Event, BytesStart}, Writer, de::Deserializer};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct QualityControlFlags {
    pub auto: Option<bool>,
    pub auto_station: Option<bool>
}

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[diesel(table_name = metars)]
pub struct Metar {
    pub raw_text: String,
    pub station_id: String,
    pub observation_time: String,
    pub latitude: f64,
    pub longitude: f64,
    pub temp_c: Option<f64>,
    pub dewpoint_c: Option<f64>,
    pub wind_dir_degrees: Option<String>,
    pub wind_speed_kt: Option<i32>,
    pub visibility_statute_mi: Option<String>,
    pub altim_in_hg: Option<f64>,
    pub sea_level_pressure_mb: Option<f64>,
    // pub quality_control_flags: Option<QualityControlFlags>,
    pub wx_string: Option<String>,
    // pub sky_con dition: Option<Vec<String>>, // TODO work on attributes
    pub flight_category: String,
    pub three_hr_pressure_tendency_mb: Option<f64>,
    pub metar_type: String,
    #[serde(rename = "maxT_c")]
    pub max_t_c: Option<f64>,
    #[serde(rename = "  ")]
    pub min_t_c: Option<f64>,
    pub precip_in: Option<f64>,
    pub elevation_m: i32
}

impl Metar {
    pub fn parse(input: String) -> Result<Vec<Self>, CustomError> {
        if input.is_empty() {
            return Err(CustomError::new(500, "Input is empty".to_string()))
        }

        let mut reader = Reader::from_str(&input);
        let mut buf = Vec::new();
        let mut junk_buf: Vec<u8> = Vec::new();
        let mut metars: Vec<Self> = vec![];

        loop {
            match reader.read_event_into(&mut buf) {
                Err(e) => panic!("Error at position: {}: {:?}", reader.buffer_position(), e),
                Ok(Event::Eof) => break,
                Ok(Event::Start(e)) => {
                    match e.name().as_ref() {
                        b"METAR" => {
                            let metar_bytes = Self::read_to_end_into_buffer(&mut reader, &e, &mut junk_buf).unwrap();
                            let str = std::str::from_utf8(&metar_bytes).unwrap();
                            let mut deserializer = Deserializer::from_str(str);
                            match Self::deserialize(&mut deserializer) {
                                Ok(m) => metars.push(m),
                                Err(err) => warn!("{}", err)
                            };
                        },
                        _ => ()
                    }
                },
                _ => ()
            }
        }

        return Ok(metars)
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

#[derive(Serialize, Deserialize, Queryable)]
pub struct Metars {
    pub id: i32,
    pub raw_text: String,
    pub station_id: String,
    pub observation_time: String,
    pub latitude: f64,
    pub longitude: f64,
    pub temp_c: Option<f64>,
    pub dewpoint_c: Option<f64>,
    pub wind_dir_degrees: Option<String>,
    pub wind_speed_kt: Option<i32>,
    pub visibility_statute_mi: Option<String>,
    pub altim_in_hg: Option<f64>,
    pub sea_level_pressure_mb: Option<f64>,
    // pub quality_control_flags: Option<QualityControlFlags>,
    pub wx_string: Option<String>,
    // pub sky_condition: Option<Vec<String>>, // TODO work on attributes
    pub flight_category: String,
    pub three_hr_pressure_tendency_mb: Option<f64>,
    pub metar_type: String,
    #[serde(rename = "maxT_c")]
    pub max_t_c: Option<f64>,
    #[serde(rename = "minT_c")]
    pub min_t_c: Option<f64>,
    pub precip_in: Option<f64>,
    pub elevation_m: i32
}

impl Metars {
    pub async fn get_all(icaos: String) -> Result<Vec<Self>, CustomError> {
        if icaos.is_empty() {
            return Ok(vec![]);
        }
        let station_icaos: Vec<&str> = icaos.split(',').collect();
        let mut conn = db::connection()?;
        let db_metars: Vec<Metars> = match metars::table
            .filter(metars::station_id.eq_any(station_icaos))
            .order(metars::id.asc())
            .load::<Metars>(&mut conn) {
                Ok(m) => m,
                Err(err) => return Err(CustomError { error_status_code: 500, error_message: format!("{}", err) })
            };
        let url = format!("https://beta.aviationweather.gov/cgi-bin/data/metar.php?ids={}&format=xml", icaos);
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
        match diesel::insert_into(metars::table).values(&metars).execute(&mut conn) {
            Ok(rows) => debug!("Inserted {} metar rows", rows),
            Err(err) => error!("Unable to insert metar data; {}", err)
        };
        let mut returned_metars: Vec<Self> = vec![];
        for metar in &metars {
            // let _ = diesel::insert_into(metars::table)
            // .values(metar)
            // .execute(&mut conn);
            returned_metars.push(Self {
                id: 0,
                raw_text: metar.raw_text.to_string(),
                station_id: metar.station_id.to_string(),
                observation_time: metar.observation_time.to_string(),
                latitude: metar.latitude,
                longitude: metar.longitude,
                temp_c: metar.temp_c,
                dewpoint_c: metar.dewpoint_c,
                wind_dir_degrees: match &metar.wind_dir_degrees {
                    Some(d) => Some(d.to_string()),
                    None => None
                },
                wind_speed_kt: metar.wind_speed_kt,
                visibility_statute_mi: match &metar.visibility_statute_mi {
                    Some(d) => Some(d.to_string()),
                    None => None
                },
                altim_in_hg: metar.altim_in_hg,
                sea_level_pressure_mb: metar.sea_level_pressure_mb,
                wx_string: match &metar.wx_string {
                    Some(d) => Some(d.to_string()),
                    None => None
                },
                flight_category: metar.flight_category.to_string(),
                three_hr_pressure_tendency_mb: metar.three_hr_pressure_tendency_mb,
                metar_type: metar.metar_type.to_string(),
                max_t_c: metar.max_t_c,
                min_t_c: metar.min_t_c,
                precip_in: metar.precip_in,
                elevation_m: metar.elevation_m,
            })
        }
        Ok(returned_metars)
    }
}