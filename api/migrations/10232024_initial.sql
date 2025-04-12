CREATE EXTENSION IF NOT EXISTS postgis;

CREATE TABLE IF NOT EXISTS airports (
    icao TEXT PRIMARY KEY NOT NULL,
    iata TEXT,
    local TEXT,
    name TEXT NOT NULL,
    category TEXT NOT NULL,
    iso_country TEXT NOT NULL,
    iso_region TEXT NOT NULL,
    municipality TEXT NOT NULL,
    elevation_ft REAL NOT NULL,
    longitude REAL NOT NULL,
    latitude REAL NOT NULL,
    has_tower BOOLEAN DEFAULT false,
    has_beacon BOOLEAN DEFAULT false,
    public BOOLEAN DEFAULT false
);

CREATE INDEX ON airports (iata);
CREATE INDEX ON airports (local);
CREATE INDEX ON airports (name);
CREATE INDEX ON airports (category);
CREATE INDEX ON airports (iso_country);
CREATE INDEX ON airports (iso_region);
CREATE INDEX ON airports (municipality);
CREATE INDEX ON airports (longitude, latitude);

CREATE TABLE IF NOT EXISTS runways (
    id UUID PRIMARY KEY NOT NULL,
    icao TEXT NOT NULL,
    runway_id TEXT NOT NULL,
    length_ft REAL NOT NULL,
    width_ft REAL NOT NULL,
    surface TEXT NOT NULL
);

CREATE INDEX ON runways (icao);
CREATE INDEX ON runways (surface);

CREATE TABLE IF NOT EXISTS frequencies (
    id UUID PRIMARY KEY NOT NULL,
    icao TEXT NOT NULL,
    frequency_id TEXT NOT NULL,
    frequency_mhz REAL NOT NULL
);

CREATE INDEX ON frequencies (icao);
CREATE INDEX ON frequencies (frequency_mhz);

CREATE TABLE IF NOT EXISTS metars (
    icao TEXT NOT NULL,
    observation_time TIMESTAMPTZ NOT NULL,
    raw_text TEXT NOT NULL,
    data JSONB NOT NULL
);

CREATE INDEX ON metars (observation_time DESC);

CREATE TABLE IF NOT EXISTS users (
    email TEXT PRIMARY KEY NOT NULL,
    password_hash TEXT NOT NULL,
    role TEXT NOT NULL,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);