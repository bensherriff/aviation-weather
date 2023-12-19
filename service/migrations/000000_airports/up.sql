CREATE EXTENSION IF NOT EXISTS postgis;
CREATE TABLE IF NOT EXISTS airports (
  icao TEXT PRIMARY KEY NOT NULL,
  category TEXT NOT NULL,
  name TEXT NOT NULL,
  elevation_ft REAL NOT NULL,
  iso_country TEXT NOT NULL,
  iso_region TEXT NOT NULL,
  municipality TEXT NOT NULL,
  iata_code TEXT NOT NULL,
  local_code TEXT NOT NULL,
  point GEOMETRY(POINT,4326) NOT NULL,
  data JSONB NOT NULL
);