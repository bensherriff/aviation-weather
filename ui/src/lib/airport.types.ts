import { Metar } from './metar.types';

export enum AirportCategory {
  SMALL = 'small_airport',
  MEDIUM = 'medium_airport',
  LARGE = 'large_airport',
  HELIPORT = 'heliport',
  BALLOONPORT = 'balloon_port',
  CLOSED = 'closed',
  SEAPLANE = 'seaplane_base',
  UNKNOWN = 'unknown'
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
  iata: string;
  local: string;
  name: string;
  category: AirportCategory;
  iso_country: string;
  iso_region: string;
  municipality: string;
  elevation_ft: number;
  latitude: number;
  longitude: number;
  has_tower: boolean;
  has_beacon: boolean;
  runways: Runway[];
  frequencies: Frequency[];
  public: boolean;
  latest_metar?: Metar;
}

export interface Runway {
  id: string;
  length_ft: number;
  width_ft: number;
  surface: string;
}

export interface Frequency {
  id: string;
  frequency_mhz: number;
}

export interface GetAirportsResponse {
  data: Airport[];
  limit: number;
  page: number;
  total: number;
}
