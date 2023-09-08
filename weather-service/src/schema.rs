diesel::table! {
  airports (id) {
    id -> Integer,
    full_name -> Text,
    icao -> Text,
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
