import axios from 'axios';
import { Airport } from './airport.types';

interface GetAirportsProps {
  bounds?: Bounds;
  category?: string;
  page?: number;
  limit?: number;
}

export interface Bounds {
  northEast: Coordinate;
  southWest: Coordinate;
}

export interface Coordinate {
  lat: number;
  lon: number;
}

interface GetAirportProps {
  icao: string;
}

export async function getAirport({ icao }: GetAirportProps) {
  const response = await axios.get(`http://localhost:5000/airports/${icao}`).catch((error) => console.error(error));
  return response?.data;
}

export async function getAirports({ bounds, category, limit = 10, page = 1 }: GetAirportsProps): Promise<Airport[]> {
  const response = await axios
    .get(`http://localhost:5000/airports`, {
      params: {
        ne_lat: bounds?.northEast.lat,
        ne_lon: bounds?.northEast.lon,
        sw_lat: bounds?.southWest.lat,
        sw_lon: bounds?.southWest.lon,
        category,
        limit,
        page
      }
    })
    .catch((error) => console.error(error));
  return response?.data || [];
}
