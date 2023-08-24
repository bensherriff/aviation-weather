import { Metar } from "./weather";

export class Airport {
    name: string;
    icao: string;
    metar: Metar | undefined;

    constructor(name: string, icao: string) {
        this.name = name;
        this.icao = icao;
        this.metar = undefined;
    }
}
