import { Metar } from './metar.types';
import { getRequest } from '.';

export async function getMetars(icaos: string[]): Promise<Metar[]> {
  if (icaos.length == 0) {
    return [];
  }
  const stationICAOs: string = icaos.map((icao) => icao).join(',');
  const response = await getRequest(`metars`, { icaos: stationICAOs });
  return response?.json() || [];
}
