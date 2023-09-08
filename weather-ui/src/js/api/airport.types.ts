import { Metar } from './metar.types';

export interface Airport {
  name: string;
  icao: string;
  latitude: number;
  longitude: number;
  metar?: Metar;
}
