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
import { useEffect, useState } from 'react';
import { Airport } from '@lib/airport.types.ts';
import AirportDrawer from '@components/AirportDrawer.tsx';
import { getWeatherMapUrl } from '@lib/rainViewer.ts';
import { Switch } from '@mantine/core';

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
const defaultZoom = 6;
const defaultCenter: L.LatLngExpression = [38.944444, -77.455833];

function App() {
  const [airport, setAirport] = useState<Airport | null>(null);
  const [rainViewerUrl, setRainViewerUrl] = useState<string | null>(null);
  const [showRadar, setShowRadar] = useState<boolean>(false);

  useEffect(() => {
    if (showRadar) {
      getWeatherMapUrl().then(url => {
        setRainViewerUrl(url);
      });
    }
  }, [showRadar])

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
          { rainViewerUrl && showRadar && (
            <TileLayer url={rainViewerUrl} opacity={0.5} zIndex={5} />
          )}
          <AirportLayer setAirport={setAirport} />
        </MapContainer>
        <div style={{
          position: 'absolute', top: '100px', right: '10px', zIndex: 1000, backgroundColor: '#fff',
          borderRadius: '8px', boxShadow: '0 2px 6px rgba(0,0,0,0.15)', padding: '10px'
        }}>
          <Switch
            label="Radar"
            checked={showRadar}
            onChange={(event) => setShowRadar(event.currentTarget.checked)}
          />
        </div>
      </div>
    </div>
  );
}

export default App;
