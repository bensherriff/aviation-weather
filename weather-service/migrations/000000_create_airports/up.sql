CREATE TABLE IF NOT EXISTS airports (
    id INTEGER PRIMARY KEY GENERATED  ALWAYS AS IDENTITY,
    icao TEXT NOT NULL,
    category TEXT NOT NULL,
    full_name TEXT NOT NULL,
    elevation_ft INTEGER,
    continent TEXT NOT NULL,
    iso_country TEXT NOT NULL,
    iso_region TEXT NOT NULL,
    municipality TEXT NOT NULL,
    gps_code TEXT NOT NULL,
    iata_code TEXT NOT NULL,
    local_code TEXT NOT NULL,
    latitude DOUBLE PRECISION NOT NULL,
    longitude DOUBLE PRECISION NOT NULL
)