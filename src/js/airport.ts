import { Metar } from "./weather";

export class Airport {
    name: string;
    icao: string;
    latitude: number;
    longitude: number;
    metar: Metar | undefined;

    constructor(name: string, icao: string) {
        this.name = name;
        this.icao = icao;
        this.latitude = 0;
        this.longitude = 0;
        this.metar = undefined;
    }
}
