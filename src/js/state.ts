import { Airport } from "./airport";

const airports: Map<string, Airport> = new Map();

export function setAirport(icao: string, airport: Airport) {
    airports.set(icao, airport);
}

export function getAirport(icao: string): Airport | undefined {
    return airports.get(icao);
}

export function getAirports(): Airport[] {
    return [...airports.values()];
}