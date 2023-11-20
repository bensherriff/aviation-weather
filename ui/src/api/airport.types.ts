import { Metadata } from '.';
import { Metar } from './metar.types';

export enum AirportCategory {
  SMALL = 'small_airport',
  MEDIUM = 'medium_airport',
  LARGE = 'large_airport'
}

export enum AirportOrderField {
  ICAO = 'icao',
  NAME = 'name',
  CATEGORY = 'category',
  CONTINENT = 'continent',
  ISO_COUNTRY = 'iso_country',
  ISO_REGION = 'iso_region',
  MUNICIPALITY = 'municipality',
  GPS_CODE = 'gps_code',
  IATA_CODE = 'iata_code',
  LOCAL_CODE = 'local_code',
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
  elevation_ft: number;
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
  meta: Metadata;
}

export interface GetAirportsResponse {
  data: Airport[];
  meta: Metadata;
}
