import { Airport, AirportOrderField, Bounds, GetAirportResponse, GetAirportsResponse } from './airport.types';
import { getRequest, deleteRequest, postRequest, putRequest } from '.';

interface GetAirportProps {
  icao: string;
}

export async function getAirport({ icao }: GetAirportProps): Promise<GetAirportResponse> {
  const response = await getRequest(`airports/${icao}`);
  return response?.json() || { data: undefined };
}

interface GetAirportsProps {
  bounds?: Bounds;
  categories?: string[];
  icaos?: string[];
  name?: string;
  order_field?: AirportOrderField;
  order_by?: 'asc' | 'desc';
  has_metar?: boolean;
  page?: number;
  limit?: number;
}

export async function getAirports({
  bounds,
  categories,
  icaos,
  name,
  order_field,
  order_by,
  has_metar,
  limit = 10,
  page = 1
}: GetAirportsProps): Promise<GetAirportsResponse> {
  const response = await getRequest('airports', {
    bounds: bounds
      ? `${bounds?.northEast.lat},${bounds?.northEast.lon},${bounds?.southWest.lat},${bounds?.southWest.lon}`
      : undefined,
    categories: categories ?? undefined,
    icaos: icaos ?? undefined,
    name: name ?? undefined,
    order_field: order_field ?? undefined,
    order_by: order_by ?? undefined,
    has_metar: has_metar ?? undefined,
    limit,
    page
  });
  return response?.json() || { data: [] };
}

export async function removeAirport({ icao }: { icao?: string }): Promise<any> {
  let response
  if (icao) {
    response = await deleteRequest(`airports/${icao}`);
  } else {
    response = await deleteRequest('airports');
  }
  return response.status == 204;
}

export async function createAirport({ airport }: { airport: Airport }): Promise<any> {
  const response = await postRequest(`airports`, airport);
  return response?.json() || { data: undefined };
}

export async function updateAirport({ airport }: { airport: Airport }): Promise<any> {
  const response = await putRequest(`airports/${airport.icao}`, airport);
  return response?.json() || { data: undefined };
}

export async function importAirports(payload: File): Promise<boolean> {
  const data = new FormData();
  data.append('data', payload);
  const response = await postRequest('airports/import', data, {
    type: 'form'
  });
  return response ? response.status === 200 : false;
}
