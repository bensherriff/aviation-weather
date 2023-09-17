import { Bounds, GetAirportResponse, GetAirportsResponse } from './airport.types';
import { getRequest } from '.';

interface GetAirportProps {
  icao: string;
}

export async function getAirport({ icao }: GetAirportProps): Promise<GetAirportResponse> {
  const response = await getRequest(`airports/${icao}`, {});
  return response?.data || { data: undefined };
}

interface GetAirportsProps {
  bounds?: Bounds;
  category?: string;
  filter?: string;
  page?: number;
  limit?: number;
}

export async function getAirports({
  bounds,
  category,
  filter,
  limit = 10,
  page = 1
}: GetAirportsProps): Promise<GetAirportsResponse> {
  const response = await getRequest('airports', {
    bounds: bounds
      ? `${bounds?.northEast.lat},${bounds?.northEast.lon},${bounds?.southWest.lat},${bounds?.southWest.lon}`
      : undefined,
    category: category ?? undefined,
    filter: filter ?? undefined,
    limit,
    page
  });
  return response?.data || { data: [] };
}
