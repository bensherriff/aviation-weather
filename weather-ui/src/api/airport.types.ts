import { Metar } from './metar.types';

export enum AirportCategory {
  SMALL = 'small_airport',
  MEDIUM = 'medium_airport',
  LARGE = 'large_airport'
}

export interface Bounds {
  northEast: Coordinate;
  southWest: Coordinate;
}

export interface Coordinate {
  lat: number;
  lon: number;
}

export interface Airport {
  icao: string;
  category: AirportCategory;
  full_name: string;
  elevation_ft: string;
  continent: string;
  iso_country: string;
  iso_region: string;
  municipality: string;
  gps_code: string;
  iata_code: string;
  local_code: string;
  point: {
    x: number;
    y: number;
    srid: number;
  };
  metar?: Metar;
}

export interface GetAirportResponse {
  data: Airport;
}

export interface GetAirportsResponse {
  data: Airport[];
}
