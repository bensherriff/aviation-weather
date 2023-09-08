import { Metar } from "./weather";

export class Airport {
    name: string;
    icao: string;
    latitude: number;
    longitude: number;
    metar: Metar | undefined;

    constructor(name: string, icao: string, latitude: number, longitude: number) {
        this.name = name;
        this.icao = icao;
        this.latitude = latitude;
        this.longitude = longitude;
        this.metar = undefined;
    }
}
