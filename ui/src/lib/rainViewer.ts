import { WeatherMaps } from '@lib/rainViewer.types.ts';

const weatherMapsUrl = 'https://api.rainviewer.com/public/weather-maps.json';

async function getWeatherMaps(): Promise<WeatherMaps | undefined> {
  const response = await fetch(`${weatherMapsUrl}`, {
    method: 'GET'
  });
  if (response?.status === 200) {
    return response.json();
  } else {
    return undefined;
  }
}

export async function getWeatherMapUrl(): Promise<string | null> {
  const weatherMaps = await getWeatherMaps();
  if (weatherMaps != undefined) {
    let url = weatherMaps.host;
    // url = 'https://cdn.rainviewer.com';
    let latest = '';
    if (weatherMaps.radar.past.length > 0) {
      latest = weatherMaps.radar.past[weatherMaps.radar.past.length - 1].path;
    } else {
      return null;
    }
    url += latest + '/256/{z}/{x}/{y}/2/1_1.png';
    // url += latest + "/256/{z}/{x}/{y}/255/1_1_1_0.webp";
    return url;
  } else {
    return null;
  }
}
