import { Metadata } from '.';
import { Metar } from './metar.types';

export enum AirportCategory {
  SMALL = 'small_airport',
  MEDIUM = 'medium_airport',
  LARGE = 'large_airport',
  HELIPORT = 'heliport',
  BALLOONPORT = 'balloonport',
  CLOSED = 'closed',
  SEAPLANE = 'seaplane_base',
  UNKNOWN = 'unknown',
}

export function airportCategoryToText(category: AirportCategory): string {
  switch (category) {
    case AirportCategory.SMALL:
      return 'Small';
    case AirportCategory.MEDIUM:
      return 'Medium';
    case AirportCategory.LARGE:
      return 'Large';
    case AirportCategory.HELIPORT:
      return 'Helipad';
    case AirportCategory.CLOSED:
      return 'Closed';
    case AirportCategory.SEAPLANE:
      return 'Seaplane Base';
    case AirportCategory.BALLOONPORT:
      return 'Balloonport';
    default:
      return 'Unknown';
  }
}

export enum AirportOrderField {
  ICAO = 'icao',
  NAME = 'name',
  CATEGORY = 'category',
  CONTINENT = 'continent',
  ISO_COUNTRY = 'iso_country',
  ISO_REGION = 'iso_region',
  MUNICIPALITY = 'municipality',
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
  name: string;
  elevation_ft: number;
  iso_country: string;
  iso_region: string;
  municipality: string;
  iata_code: string;
  local_code: string;
  latitude: number;
  longitude: number;
  latest_metar?: Metar;
}

export interface GetAirportResponse {
  data: Airport;
  meta: Metadata;
}

export interface GetAirportsResponse {
  data: Airport[];
  meta: Metadata;
}
