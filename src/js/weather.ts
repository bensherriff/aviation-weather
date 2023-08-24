/* eslint-disable @typescript-eslint/no-unsafe-argument */
/* eslint-disable @typescript-eslint/no-unsafe-assignment */
/* eslint-disable @typescript-eslint/no-unsafe-member-access */
import axios from 'axios';
import { xml2json } from 'xml-js';
import { Airport } from './airport';

const base_url = 'https://beta.aviationweather.gov/cgi-bin/data';

export async function getMetars(airports: Airport[]): Promise<Metar[]> {
    const stationICAOs: string = airports
        .map((airport) => airport.icao)
        .join(',');
    const url = `${base_url}/metar.php?ids=${stationICAOs}&format=xml`;
    const response = await axios
        .get(`${url}`)
        .catch((error) => console.error(`${error}`));
    // const metars = new Map<string, Metar>();
    const metars: Metar[] = [];
    if (response != null && response != undefined) {
        const json = xml2json(response.data, { compact: true });
        const jsonObject = JSON.parse(json);
        let metarData = jsonObject?.response?.data?.METAR;
        if (!Array.isArray(metarData)) {
            metarData = [metarData];
        }
        for (const data of metarData) {
            const sky_condition: {
                sky_cover: string;
                cloud_base_ft_agl: number;
            }[] = [];
            if (Array.isArray(data.sky_condition)) {
                for (const sc of data.sky_condition) {
                    sky_condition.push({
                        sky_cover: sc.sky_cover,
                        cloud_base_ft_agl: Number(sc.cloud_base_ft_agl)
                    })
                }
            } else {
                sky_condition.push({
                    sky_cover: data.sky_condition?.sky_cover,
                    cloud_base_ft_agl: Number(data.sky_condition?.cloud_base_ft_agl)
                })
            }
            const metar: Metar = {
                raw_text: data.raw_text._text,
                station_id: data.station_id._text,
                observation_time: data.observation_time._text,
                latitude: Number(data.latitude._text),
                longitude: Number(data.longitude._text),
                temp_c: Number(data.temp_c._text),
                dewpoint_c: Number(data.dewpoint_c._text),
                wind_dir_degrees: Number(data.wind_dir_degrees._text),
                wind_speed_kt: Number(data.wind_speed_kt._text),
                visibility_statute_mi: data.visibility_statute_mi._text,
                altim_in_hg: Number(data.altim_in_hg._text),
                sea_level_pressure_mb: data.sea_level_pressure_mb?._text,
                quality_control_flags: {
                    auto: data.quality_control_flags?.auto?._text == 'TRUE',
                    auto_station: data.quality_control_flags?.auto_station?._text == 'TRUE',
                },
                wx_string: data.wx_string?._text,
                sky_condition: sky_condition,
                flight_category: data.flight_category._text,
                three_hr_pressure_tendency_mb: data.three_hr_pressure_tendency_mb?._text,
                metar_type: data.metar_type._text,
                maxT_c: Number(data.maxT_c?._text),
                minT_c: Number(data.minT_c?._text),
                precip_in: Number(data.precip_in?._text),
                elevation_m: Number(data.elevation_m._text),
            };
            metars.push(metar);
        }
    }
    return metars;
}

export interface Metar {
    raw_text: string;
    station_id: string;
    observation_time: string;
    latitude: number;
    longitude: number;
    temp_c: number;
    dewpoint_c: number;
    wind_dir_degrees: number;
    wind_speed_kt: number;
    visibility_statute_mi: string;
    altim_in_hg: number;
    sea_level_pressure_mb: number;
    quality_control_flags: {
        auto: boolean;
        auto_station: boolean;
    };
    wx_string: string;
    sky_condition: {
        sky_cover: string;
        cloud_base_ft_agl: number;
    }[];
    flight_category: string;
    three_hr_pressure_tendency_mb: number;
    metar_type: string;
    maxT_c: number;
    minT_c: number;
    precip_in: number;
    elevation_m: number;
}
