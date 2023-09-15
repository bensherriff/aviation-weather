import axios from 'axios';
import { Airport } from './airport.types';
import { Metar } from './metar.types';

interface GetMetarsResponse {
  data: Metar[];
}

export async function getMetars(airports: Airport[]): Promise<GetMetarsResponse> {
  if (airports.length == 0) {
    return { data: [] };
  }
  const stationICAOs: string = airports.map((airport) => airport.icao).join(',');
  const url = `http://localhost:5000/metars/${stationICAOs}`;
  const response = await axios.get(url).catch((error) => console.error(error));
  return response?.data || { data: [] };
}
