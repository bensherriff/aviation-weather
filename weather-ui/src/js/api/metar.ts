import axios from 'axios';
import { Airport } from './airport.types';
import { Metar } from './metar.types';

export async function getMetars(airports: Airport[]): Promise<Metar[]> {
  const stationICAOs: string = airports.map((airport) => airport.icao).join(',');
  const url = `http://localhost:5000/metars/${stationICAOs}`;
  const response = await axios.get(url).catch((error) => console.error(error));
  return response?.data || [];
}
