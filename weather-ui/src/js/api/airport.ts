import axios from 'axios';
import { Airport } from './airport.types';

interface GetAirportsProps {
  ne_lat: number;
  ne_lon: number;
  sw_lat: number;
  sw_lon: number;
  page?: number;
  limit?: number;
}

interface GetAirportProps {
  icao: string;
}

export async function getAirport({ icao }: GetAirportProps) {
  const response = await axios.get(`http://localhost:5000/airports/${icao}`).catch((error) => console.error(error));
  return response?.data;
}

export async function getAirports({
  ne_lat,
  ne_lon,
  sw_lat,
  sw_lon,
  limit = 10,
  page = 1
}: GetAirportsProps): Promise<Airport[]> {
  const response = await axios
    .get(`http://localhost:5000/airports`, { params: { ne_lat, ne_lon, sw_lat, sw_lon, page, limit } })
    .catch((error) => console.error(error));
  return response?.data;
}
