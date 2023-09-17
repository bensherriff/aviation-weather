import { Airport } from './airport.types';
import { Metar } from './metar.types';
import { getRequest } from '.';

interface GetMetarsResponse {
  data: Metar[];
}

export async function getMetars(airports: Airport[]): Promise<GetMetarsResponse> {
  if (airports.length == 0) {
    return { data: [] };
  }
  const stationICAOs: string = airports.map((airport) => airport.icao).join(',');
  const response = await getRequest(`metars/${stationICAOs}`, {});
  return response?.data || { data: [] };
}
