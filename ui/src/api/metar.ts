import { Metar } from './metar.types';
import { getRequest } from '.';

interface GetMetarsResponse {
  data: Metar[];
}

export async function getMetars(icaos: string[]): Promise<GetMetarsResponse> {
  if (icaos.length == 0) {
    return { data: [] };
  }
  const stationICAOs: string = icaos.map((icao) => icao).join(',');
  const response = await getRequest(`metars/${stationICAOs}`, {});
  return response?.json() || { data: [] };
}
