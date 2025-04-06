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

CREATE TABLE IF NOT EXISTS metars (
    icao TEXT NOT NULL,
    observation_time TIMESTAMPTZ NOT NULL,
    raw_text TEXT NOT NULL,
    data JSONB NOT NULL
);

CREATE TABLE IF NOT EXISTS users (
    email TEXT PRIMARY KEY NOT NULL,
    password_hash TEXT NOT NULL,
    role TEXT NOT NULL,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);