export interface SkyCondition {
  sky_cover: string;
  cloud_base_ft_agl: number;
}

export interface QualityControlFlags {
  auto: boolean;
  auto_station_without_precipitation: boolean;
  auto_station_with_precipication: boolean;
  maintenance_indicator_on: boolean;
  corrected: boolean;
}

export interface RunwayVisualRange {
  runway: string;
  visibility_ft: string;
  variable_visibility_high_ft: string;
  variable_visibility_low_ft: string;
}

export interface Metar {
  raw_text: string;
  station_id: string;
  observation_time: string;
  temp_c: number;
  dewpoint_c: number;
  wind_dir_degrees: string;
  wind_speed_kt: number;
  wind_gust_kt: number;
  variable_wind_dir_degrees: string;
  visibility_statute_mi: string;
  runway_visual_range: RunwayVisualRange[];
  altim_in_hg: number;
  sea_level_pressure_mb: number;
  quality_control_flags: QualityControlFlags;
  weather_phenomena: string[];
  sky_condition: SkyCondition[];
  flight_category: 'VFR' | 'MVFR' | 'LIFR' | 'IFR' | 'UNKN';
  three_hr_pressure_tendency_mb: number;
  maxT_c: number;
  minT_c: number;
  precip_in: number;
}
