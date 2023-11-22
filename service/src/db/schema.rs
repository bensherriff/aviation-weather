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
    station_id -> Text,
    observation_time -> Timestamp,
    raw_text -> Text,
    data -> Jsonb,
  }
}

diesel::table! {
  users (email) {
    email -> Text,
    hash -> Text,
    role -> Text,
    first_name -> Text,
    last_name -> Text,
    updated_at -> Timestamp,
    created_at -> Timestamp,
    profile_picture -> Nullable<Text>,
    favorites -> Array<Text>,
    verified -> Bool,
  }
}
