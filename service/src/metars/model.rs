use crate::{error_handler::ServiceError, db};
use crate::db::schema::metars::{self};
use diesel::{prelude::*, sql_query};
use log::{warn, trace};
use std::collections::HashSet;
use std::io::BufRead;
use quick_xml::{Reader, events::{Event, BytesStart}, Writer, de::Deserializer};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct QualityControlFlags {
    pub auto: Option<bool>,
    pub auto_station: Option<bool>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SkyCondition {
    #[serde(rename(deserialize = "@sky_cover"))]
    pub sky_cover: String,
    #[serde(rename(deserialize = "@cloud_base_ft_agl"))]
    pub cloud_base_ft_agl: Option<i32>
}

#[derive(Serialize, Deserialize, Debug)]
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
    pub quality_control_flags: Option<QualityControlFlags>,
    pub wx_string: Option<String>,
    pub sky_condition: Option<Vec<SkyCondition>>,
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

impl Metar {
    fn parse(input: String) -> Result<Vec<Self>, ServiceError> {
        if input.is_empty() {
            return Err(ServiceError::new(500, "Input is empty".to_string()))
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
                                Err(err) => warn!("Error deserializing; {}", err)
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
    fn read_to_end_into_buffer<R: BufRead>(reader: &mut Reader<R>, start_tag: &BytesStart, junk_buf: &mut Vec<u8>) -> Result<Vec<u8>, quick_xml::Error> {
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

    fn get_missing_metar_icaos(db_metars: &Vec<Self>, station_icaos: &Vec<&str>) -> Vec<String> {
        let mut missing_metar_icaos: Vec<String> = vec![];
        let current_time = chrono::Local::now().naive_local().timestamp();
        let db_metars_set: HashSet<&str> = db_metars.iter().map(|icao| icao.station_id.as_str()).collect();
        let station_icaos_set: HashSet<&str> = station_icaos.to_owned().into_iter().collect();
        for difference in db_metars_set.symmetric_difference(&station_icaos_set) {
            missing_metar_icaos.push(difference.to_string());
        }
        for metar in db_metars {
            match chrono::NaiveDateTime::parse_and_remainder(&metar.observation_time, "%Y-%m-%dT%H:%M:%S") {
                Ok((time, _)) => {
                    if current_time > (time.timestamp() + 3600) {
                        trace!("{} METAR data is outdated", metar.station_id);
                        missing_metar_icaos.push(metar.station_id.to_string());
                    }
                },
                Err(err) => {
                    warn!("Parsing METAR timestamp failed; {}", err);
                    missing_metar_icaos.push(metar.station_id.to_string());
                }
            };
        }
        return missing_metar_icaos;
    }

    async fn get_remote_metars(icaos: String) -> Vec<Metar> {
        let url = format!("https://beta.aviationweather.gov/cgi-bin/data/metar.php?ids={}&format=xml", icaos);
        match reqwest::get(url).await {
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
        }
    }

    fn from_query(query_metars: Vec<QueryMetar>) -> Vec<Self> {
        let mut metars: Vec<Metar> = vec![];
        for metar in query_metars {
            let quality_control_flags = Some(QualityControlFlags {
                auto: metar.qcf_auto,
                auto_station: metar.qcf_auto_station
            });
            let sky_condition = match metar.sky_condition {
                Some(s) => {
                    let mut sc: Vec<SkyCondition> = vec![];
                    for string in s {
                        let split: Vec<&str> = string.split_whitespace().collect();
                        if split.len() == 1 {
                            sc.push(SkyCondition { sky_cover: split[0].to_string(), cloud_base_ft_agl: None })
                        } else if split.len() == 2 {
                            let cloud_base = split[1].parse::<i32>().unwrap();
                            sc.push(SkyCondition { sky_cover: split[0].to_string(), cloud_base_ft_agl: Some(cloud_base) })
                        }
                    }
                    Some(sc)
                },
                None => None
            };
            metars.push(Metar {
                raw_text: metar.raw_text,
                station_id: metar.station_id,
                observation_time: metar.observation_time,
                latitude: metar.latitude,
                longitude: metar.longitude,
                temp_c: metar.temp_c,
                dewpoint_c: metar.dewpoint_c,
                wind_dir_degrees: metar.wind_dir_degrees,
                wind_speed_kt: metar.wind_speed_kt,
                visibility_statute_mi: metar.visibility_statute_mi,
                altim_in_hg: metar.altim_in_hg,
                sea_level_pressure_mb: metar.sea_level_pressure_mb,
                quality_control_flags,
                wx_string: metar.wx_string,
                sky_condition,
                flight_category: metar.flight_category,
                three_hr_pressure_tendency_mb: metar.three_hr_pressure_tendency_mb,
                metar_type: metar.metar_type,
                max_t_c: metar.max_t_c,
                min_t_c: metar.min_t_c,
                precip_in: metar.precip_in,
                elevation_m: metar.elevation_m
            })
        }
        return metars;
    }

    fn to_insert(metars: &Vec<Self>) -> Vec<InsertMetar> {
        let mut insert_metars: Vec<InsertMetar> = vec![];
        for metar in metars {
            insert_metars.push(InsertMetar {
                raw_text: metar.raw_text.to_string(),
                station_id: metar.station_id.to_string(),
                observation_time: metar.observation_time.to_string(),
                latitude: metar.latitude,
                longitude: metar.longitude,
                temp_c: metar.temp_c,
                dewpoint_c: metar.dewpoint_c,
                wind_dir_degrees: match &metar.wind_dir_degrees {
                    Some(m) => Some(m.to_string()),
                    None => None
                },
                wind_speed_kt: metar.wind_speed_kt,
                visibility_statute_mi: match &metar.visibility_statute_mi {
                    Some(m) => Some(m.to_string()),
                    None => None
                },
                altim_in_hg: metar.altim_in_hg,
                sea_level_pressure_mb: metar.sea_level_pressure_mb,
                qcf_auto: match &metar.quality_control_flags {
                    Some(m) => m.auto,
                    None => None
                },
                qcf_auto_station: match &metar.quality_control_flags {
                    Some(m) => m.auto_station,
                    None => None
                },
                wx_string: match &metar.wx_string {
                    Some(m) => Some(m.to_string()),
                    None => None
                },
                sky_condition: match &metar.sky_condition {
                    Some(s) => {
                        let mut sc: Vec<String> = vec![];
                        for condition in s {
                            if let Some(cloud_base) = condition.cloud_base_ft_agl {
                                sc.push(format!("{} {}", condition.sky_cover, cloud_base));
                            } else {
                                sc.push(format!("{}", condition.sky_cover));
                            }
                        }
                        Some(sc)
                    },
                    None => None
                },
                flight_category: metar.flight_category.to_string(),
                three_hr_pressure_tendency_mb: metar.three_hr_pressure_tendency_mb,
                metar_type: metar.metar_type.to_string(),
                max_t_c: metar.max_t_c,
                min_t_c: metar.min_t_c,
                precip_in: metar.precip_in,
                elevation_m: metar.elevation_m
            });
        }
        return insert_metars;
    }

    pub async fn get_all(icaos: String) -> Result<Vec<Self>, ServiceError> {
        if icaos.is_empty() {
            return Ok(vec![]);
        }

        let station_icaos: Vec<&str> = icaos.split(',').collect();
        let mut db_metars = match QueryMetar::get_all(&station_icaos) {
            Ok(m) => Self::from_query(m),
            Err(err) => return Err(err)
        };

        let missing_icaos = Self::get_missing_metar_icaos(&db_metars, &station_icaos);
        if missing_icaos.is_empty() {
            return Ok(db_metars);
        }
        trace!("Retrieving missing METAR data for {:?}", missing_icaos);
        let missing_icaos_string: Vec<String> = missing_icaos.iter().map(|icao| format!("'{}'", icao.to_string())).collect();
        let mut missing_metars = Self::get_remote_metars(missing_icaos_string.join(",")).await;
        if missing_metars.len() > 0 {
            let insert_metars = Self::to_insert(&missing_metars);
            let mut conn = db::connection()?;
            match diesel::insert_into(metars::table).values(&insert_metars).execute(&mut conn) {
                Ok(rows) => trace!("Inserted {} metar rows", rows),
                Err(err) => warn!("Unable to insert metar data; {}", err)
            };
        }
        let mut metars: Vec<Metar> = vec![];
        metars.append(&mut missing_metars);
        metars.append(&mut db_metars);
        Ok(metars)
    }
}

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[diesel(table_name = metars)]
struct InsertMetar {
    raw_text: String,
    station_id: String,
    observation_time: String,
    latitude: f64,
    longitude: f64,
    temp_c: Option<f64>,
    dewpoint_c: Option<f64>,
    wind_dir_degrees: Option<String>,
    wind_speed_kt: Option<i32>,
    visibility_statute_mi: Option<String>,
    altim_in_hg: Option<f64>,
    sea_level_pressure_mb: Option<f64>,
    qcf_auto: Option<bool>,
    qcf_auto_station: Option<bool>,
    wx_string: Option<String>,
    sky_condition: Option<Vec<String>>,
    flight_category: String,
    three_hr_pressure_tendency_mb: Option<f64>,
    metar_type: String,
    #[serde(rename = "maxT_c")]
    max_t_c: Option<f64>,
    #[serde(rename = "minT_c")]
    min_t_c: Option<f64>,
    precip_in: Option<f64>,
    elevation_m: i32
}

#[derive(Serialize, Deserialize, Queryable, QueryableByName)]
#[diesel(table_name = metars)]
struct QueryMetar {
    id: i32,
    raw_text: String,
    station_id: String,
    observation_time: String,
    latitude: f64,
    longitude: f64,
    temp_c: Option<f64>,
    dewpoint_c: Option<f64>,
    wind_dir_degrees: Option<String>,
    wind_speed_kt: Option<i32>,
    visibility_statute_mi: Option<String>,
    altim_in_hg: Option<f64>,
    sea_level_pressure_mb: Option<f64>,
    qcf_auto: Option<bool>,
    qcf_auto_station: Option<bool>, 
    wx_string: Option<String>,
    sky_condition: Option<Vec<String>>,
    flight_category: String,
    three_hr_pressure_tendency_mb: Option<f64>,
    metar_type: String,
    #[serde(rename = "maxT_c")]
    max_t_c: Option<f64>,
    #[serde(rename = "minT_c")]
    min_t_c: Option<f64>,
    precip_in: Option<f64>,
    elevation_m: i32
}

impl QueryMetar {
    fn get_all(icaos: &Vec<&str>) -> Result<Vec<QueryMetar>, ServiceError> {
        let station_query: Vec<String> = icaos.iter().map(|icao| format!("'{}'", icao.to_string())).collect();
        
        let mut conn = db::connection()?;
        let db_metars: Vec<Self> = match sql_query(format!("SELECT DISTINCT ON (station_id) * FROM metars WHERE station_id IN ({}) ORDER BY station_id, observation_time DESC", station_query.join(","))).load(&mut conn) {
            Ok(m) => m,
            Err(err) => return Err(ServiceError { status: 500, message: format!("{}", err) })
        };
        return Ok(db_metars);
    }
}
