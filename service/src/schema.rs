diesel::table! {
  use diesel::sql_types::*;
  use postgis_diesel::sql_types::*;
  airports (icao) {
    icao -> Text,
    id -> Integer,
    category -> Text,
    full_name -> Text,
    elevation_ft -> Nullable<Integer>,
    continent -> Text,
    iso_country -> Text,
    iso_region -> Text,
    municipality -> Text,
    gps_code -> Text,
    iata_code -> Text,
    local_code -> Text,
    point -> Geometry,
  }
}

diesel::table! {
  metars (id) {
    id -> Integer,
    raw_text -> Text,
    station_id -> Text,
    observation_time -> Text,
    latitude -> Double,
    longitude -> Double,
    temp_c -> Nullable<Double>,
    dewpoint_c -> Nullable<Double>,
    wind_dir_degrees -> Nullable<Text>,
    wind_speed_kt -> Nullable<Integer>,
    visibility_statute_mi -> Nullable<Text>,
    altim_in_hg -> Nullable<Double>,
    sea_level_pressure_mb -> Nullable<Double>,
    qcf_auto -> Nullable<Bool>,
    qcf_auto_station -> Nullable<Bool>,
    wx_string -> Nullable<Text>,
    sky_condition -> Nullable<Array<Text>>,
    flight_category -> Text,
    three_hr_pressure_tendency_mb -> Nullable<Double>,
    metar_type -> Text,
    max_t_c -> Nullable<Double>,
    min_t_c -> Nullable<Double>,
    precip_in -> Nullable<Double>,
    elevation_m -> Integer,
  }
}

diesel::table! {
  use diesel::sql_types::*;
  use crate::users::PgUserType;
  users (id) {
    id -> Uuid,
    first_name -> Text,
    last_name -> Text,
    user_type -> PgUserType,
    favorites -> Array<Text>
  }
}
