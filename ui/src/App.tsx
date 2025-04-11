import { MapContainer, TileLayer, ZoomControl } from 'react-leaflet';
import '@mantine/core/styles.css';
import 'leaflet/dist/leaflet.css';
import './App.css';
import markerIcon2x from 'leaflet/dist/images/marker-icon-2x.png';
import markerIcon from 'leaflet/dist/images/marker-icon.png';
import markerShadow from 'leaflet/dist/images/marker-shadow.png';
import L from 'leaflet';
import { Header } from '@components/Header';
import AirportLayer from '@components/AirportLayer.tsx';
import { useState } from 'react';
import { Airport } from '@lib/airport.types.ts';
import AirportDrawer from '@components/AirportDrawer.tsx';

// Fix Leaflet's default icon path issues with Webpack
// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-expect-error
delete L.Icon.Default.prototype._getIconUrl;

L.Icon.Default.mergeOptions({
  iconRetinaUrl: markerIcon2x,
  iconUrl: markerIcon,
  shadowUrl: markerShadow
});

const openStreetMapUrl = 'https://tile.openstreetmap.org/{z}/{x}/{y}.png';
// const rainViewerUrl = 'https://tilecache.rainviewer.com/v2/radar/{time}/256/10/290/391/2/1_1.png'
// https://api.rainviewer.com/public/weather-maps.json
const defaultZoom = 6;
const defaultCenter: L.LatLngExpression = [38.944444, -77.455833];

function App() {
  const [airport, setAirport] = useState<Airport | null>(null);
  return (
    <div className='App'>
      <Header />
      <div className='map-wrapper'>
        <AirportDrawer airport={airport} setAirport={setAirport} />
        <MapContainer
          className='leaflet-container'
          attributionControl={false}
          center={defaultCenter}
          zoom={defaultZoom}
          minZoom={3}
          maxZoom={19}
          maxBounds={[
            [-85.06, -181],
            [85.06, 181]
          ]}
          scrollWheelZoom={true}
          zoomControl={false}
        >
          <ZoomControl position={'bottomright'} />
          <TileLayer url={openStreetMapUrl} />
          <AirportLayer setAirport={setAirport} />
        </MapContainer>
      </div>
    </div>
  );
}

export default App;
