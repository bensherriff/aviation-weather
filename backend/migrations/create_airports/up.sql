CREATE TABLE "airports" (
    id SERIAL PRIMARY KEY,
    full_name TEXT NOT NULL,
    icao TEXT NOT NULL,
    latitude DOUBLE PRECISION NOT NULL,
    longitude DOUBLE PRECISION NOT NULL
)