CREATE TABLE "airports" (
    id SERIAL PRIMARY KEY,
    full_name VARCHAR NOT NULL,
    icao VARCHAR NOT NULL,
    latitude INT NOT NULL,
    longitude INT NOT NULL
)