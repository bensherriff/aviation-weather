import axios from 'axios';
import { Airport } from './airport.types';

export async function getAirports(): Promise<Airport[]> {
  const response = await axios.get(`http://localhost:5000/airports`).catch((error) => console.error(error));
  return response?.data;
}
