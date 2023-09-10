diesel::table! {
  airports (id) {
    id -> Integer,
    icao -> Text,
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
    latitude -> Double,
    longitude -> Double,
  }
}

diesel::table! {
  metars (id) {
    id -> Integer,
    icao -> Text,
    raw_text -> Text,
    station_id -> Text,
    observation_time -> Text,
    latitude -> Double,
    longitude -> Double,
    temp_c -> Double,
    dewpoint_c -> Double,
    wind_dir_degrees -> Integer,
    wind_speed_kt -> Integer,
    visibility_statute_mi -> Text,
    altim_in_hg -> Double,
    sea_level_pressure_mb -> Nullable<Double>,
    wx_string -> Nullable<Text>,
    flight_category -> Text,
    three_hr_pressure_tendency_mb -> Nullable<Double>,
    metar_type -> Text,
    max_t_c -> Nullable<Double>,
    min_t_c -> Nullable<Double>,
    precip_in -> Nullable<Double>,
    elevation_m -> Integer,
  }
}
