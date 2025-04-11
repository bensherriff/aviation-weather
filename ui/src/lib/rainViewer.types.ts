export interface WeatherMaps {
  version: string;
  generated: number;
  host: string;
  radar: RadarObject;
  satellite: SatelliteObject;
}

export interface RadarObject {
  past: FrameObject[];
  nowcast: FrameObject[];
}

export interface SatelliteObject {
  infrared: FrameObject[];
}

export interface FrameObject {
  time: number;
  path: string;
}