import { AirportOrderField, Bounds, GetAirportResponse, GetAirportsResponse } from './airport.types';
import { getRequest, deleteRequest } from '.';

interface GetAirportProps {
  icao: string;
}

export async function getAirport({ icao }: GetAirportProps): Promise<GetAirportResponse> {
  const response = await getRequest(`airports/search/${icao}`);
  return response?.json() || { data: undefined };
}

interface GetAirportsProps {
  bounds?: Bounds;
  category?: string;
  name?: string;
  order_field?: AirportOrderField;
  order_by?: 'asc' | 'desc';
  icao?: string;
  page?: number;
  limit?: number;
}

export async function getAirports({
  bounds,
  category,
  name,
  icao,
  order_field,
  order_by,
  limit = 10,
  page = 1
}: GetAirportsProps): Promise<GetAirportsResponse> {
  const response = await getRequest('airports/search', {
    bounds: bounds
      ? `${bounds?.northEast.lat},${bounds?.northEast.lon},${bounds?.southWest.lat},${bounds?.southWest.lon}`
      : undefined,
    category: category ?? undefined,
    name: name ?? undefined,
    icao: icao ?? undefined,
    order_field: order_field ?? undefined,
    order_by: order_by ?? undefined,
    limit,
    page
  });
  return response?.json() || { data: [] };
}

export async function removeAirport({ icao }: { icao?: string }): Promise<any> {
  let response
  if (icao) {
    response = await deleteRequest(`airports/remove/${icao}`);
  } else {
    response = await deleteRequest('airports/remove');
  }
  return response.status == 204;
}

export async function importAirports(): Promise<any> {
  const response = await getRequest('airports/import');
  return response?.json() || { data: undefined };
}
