import { LayersControl, MapContainer, TileLayer, useMapEvents, ZoomControl } from 'react-leaflet';
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
import { IconRadar } from '@tabler/icons-react';
import Cookies from 'js-cookie';
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
const lightLayerUrl = 'https://{s}.basemaps.cartocdn.com/light_all/{z}/{x}/{y}.png';
const darkLayerUrl = 'https://{s}.basemaps.cartocdn.com/dark_all/{z}/{x}/{y}.png';
// const dark1Url = 'https://maps.rainviewer.com/data/v3/5/10/11.pbf';
// const dark2Url = 'https://basemaps.arcgis.com/arcgis/rest/services/World_Basemap_v2/VectorTileServer/tile/2/0/3.pbf';
const defaultZoom = 6;
const defaultCenter: L.LatLngExpression = [38.944444, -77.455833];

function App() {
  const [airport, setAirport] = useState<Airport | null>(null);
  const [rainViewerUrl, setRainViewerUrl] = useState<string | null>(null);
  const initialRadarValue = Cookies.get('showRadar') === 'true';
  const [showRadar, setShowRadar] = useState<boolean>(initialRadarValue);
  const [baseLayer, setBaseLayer] = useState<string>(Cookies.get('selectedBaseLayer') || 'Open Street Map');

  useEffect(() => {
    if (showRadar) {
      getWeatherMapUrl().then((url) => {
        setRainViewerUrl(url);
      });
    }
  }, [showRadar]);

  function toggleRadar() {
    setShowRadar((prev) => {
      const newValue = !prev;
      Cookies.set('showRadar', newValue.toString(), { expires: 7 });
      return newValue;
    });
  }

  function BaseLayerChangeHandler() {
    useMapEvents({
      baselayerchange: (e) => {
        setBaseLayer(e.name);
        Cookies.set('selectedBaseLayer', e.name, { expires: 7 });
      }
    });
    return null;
  }

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
          <LayersControl>
            <LayersControl.BaseLayer checked={baseLayer === 'Open Street Map'} name={'Open Street Map'}>
              <TileLayer url={openStreetMapUrl} />
            </LayersControl.BaseLayer>
            <LayersControl.BaseLayer checked={baseLayer === 'Carto Light'} name={'Carto Light'}>
              <TileLayer url={lightLayerUrl} />
            </LayersControl.BaseLayer>
            <LayersControl.BaseLayer checked={baseLayer === 'Carto Dark'} name={'Carto Dark'}>
              <TileLayer url={darkLayerUrl} />
            </LayersControl.BaseLayer>
          </LayersControl>
          {rainViewerUrl && showRadar && <TileLayer url={rainViewerUrl} opacity={0.5} zIndex={10} />}
          <ZoomControl position={'bottomright'} />
          <AirportLayer setAirport={setAirport} />
          <BaseLayerChangeHandler />
        </MapContainer>
        <IconRadar
          onClick={toggleRadar}
          style={{ bottom: '80px' }}
          className={`map-button ${showRadar ? 'active' : ''}`}
        />
      </div>
    </div>
  );
}

export default App;
