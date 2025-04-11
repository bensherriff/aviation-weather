import { WeatherMaps } from '@lib/rainViewer.types.ts';

const weatherMapsUrl = 'https://api.rainviewer.com/public/weather-maps.json';

async function getWeatherMaps(): Promise<WeatherMaps | undefined> {
  const response = await fetch(`${weatherMapsUrl}`, {
    method: 'GET',
  });
  if (response?.status === 200) {
    return response.json();
  } else {
    return undefined;
  }
}

// const rainViewerUrl = 'https://tilecache.rainviewer.com/v2/radar/1744386000/256/{z}/{x}/{y}/2/1_1.png';
// const rainViewerUrl = 'https://tilecache.rainviewer.com/v2/radar/1744386000/256/10/290/391/2/1_1.png'
// https://api.rainviewer.com/public/weather-maps.json
export async function getWeatherMapUrl(): Promise<string | null> {
  const weatherMaps = await getWeatherMaps();
  if (weatherMaps != undefined) {
    let url = weatherMaps.host;
    let latest = "";
    if (weatherMaps.radar.nowcast.length > 0) {
      latest = weatherMaps.radar.nowcast[weatherMaps.radar.nowcast.length - 1].path;
    } else if (weatherMaps.radar.past.length > 0) {
      latest = weatherMaps.radar.past[weatherMaps.radar.past.length - 1].path;
    } else {
      return null;
    }
    url += latest + "/256/{z}/{x}/{y}/2/1_1.png";
    return url;
  } else {
    return null;
  }
}