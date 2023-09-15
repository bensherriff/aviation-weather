import axios from 'axios';
import { Bounds, GetAirportResponse, GetAirportsResponse } from './airport.types';

interface GetAirportProps {
  icao: string;
}

export async function getAirport({ icao }: GetAirportProps): Promise<GetAirportResponse> {
  const response = await axios.get(`http://localhost:5000/airports/${icao}`).catch((error) => console.error(error));
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
  const response = await axios
    .get(`http://localhost:5000/airports`, {
      params: {
        bounds: bounds
          ? `${bounds?.northEast.lat},${bounds?.northEast.lon},${bounds?.southWest.lat},${bounds?.southWest.lon}`
          : undefined,
        category: category ?? undefined,
        filter: filter ?? undefined,
        limit,
        page
      }
    })
    .catch((error) => console.error(error));
  return response?.data || { data: [] };
}
