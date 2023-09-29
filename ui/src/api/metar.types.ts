export interface SkyCondition {
  sky_cover: string;
  cloud_base_ft_agl: number;
}

export interface QualityControlFlags {
  auto: boolean;
  auto_station: boolean;
}

export interface Metar {
  raw_text: string;
  station_id: string;
  observation_time: string;
  latitude: number;
  longitude: number;
  temp_c: number;
  dewpoint_c: number;
  wind_dir_degrees: string;
  wind_speed_kt: number;
  visibility_statute_mi: string;
  altim_in_hg: number;
  sea_level_pressure_mb: number;
  quality_control_flags: QualityControlFlags;
  wx_string: string;
  sky_condition: SkyCondition[];
  flight_category: 'VFR' | 'MVFR' | 'LIFR' | 'IFR' | 'UNKN';
  three_hr_pressure_tendency_mb: number;
  metar_type: string;
  maxT_c: number;
  minT_c: number;
  precip_in: number;
  elevation_m: number;
}
