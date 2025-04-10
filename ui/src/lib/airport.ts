import { Airport, AirportCategory, Bounds, GetAirportsResponse } from '@lib/airport.types.ts';
import { getRequest } from '@lib/index.ts';

export async function getAirport({ icao }: { icao: string }): Promise<Airport> {
  const response = await getRequest(`airports/${icao}`);
  return response?.json() || {};
}

interface GetAirportsParameters {
  icaos?: string[];
  name?: string;
  categories?: AirportCategory[];
  bounds?: Bounds;
  metars?: boolean;
  page?: number;
  limit?: number;
}

export async function getAirports({
  icaos,
  name,
  categories,
  bounds,
  metars = false,
  limit = 1000,
  page = 1
}: GetAirportsParameters): Promise<GetAirportsResponse> {
  const response = await getRequest('airports', {
    bounds: bounds
      ? `${bounds?.northEast.lat},${bounds?.northEast.lon},${bounds?.southWest.lat},${bounds?.southWest.lon}`
      : undefined,
    categories: categories ?? undefined,
    icaos: icaos ?? undefined,
    name: name ?? undefined,
    metars: metars ?? undefined,
    limit,
    page
  });
  return response?.json() || { data: [] };
}
