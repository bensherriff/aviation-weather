import axios from 'axios';
import { Airport } from './airport.types';

interface GetAirportsProps {
  page: number;
  limit: number;
}

interface GetAirportProps {
  icao: string;
}

export async function getAirport({ icao }: GetAirportProps) {
  const response = await axios.get(`http://localhost:5000/airports/${icao}`).catch((error) => console.error(error));
  return response?.data;
}

export async function getAirports({ limit = 10, page = 1 }: GetAirportsProps): Promise<Airport[]> {
  const response = await axios
    .get(`http://localhost:5000/airports`, { params: { page: page, limit: limit } })
    .catch((error) => console.error(error));
  return response?.data;
}
