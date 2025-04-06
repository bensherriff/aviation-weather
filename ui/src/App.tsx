import { MapContainer, TileLayer } from 'react-leaflet';
import '@mantine/core/styles.css';
import 'leaflet/dist/leaflet.css';
import './App.css';
import markerIcon2x from 'leaflet/dist/images/marker-icon-2x.png';
import markerIcon from 'leaflet/dist/images/marker-icon.png';
import markerShadow from 'leaflet/dist/images/marker-shadow.png';
// import { Header } from '@components/Header';

// Fix for default marker icon issues in React-Leaflet
import L from 'leaflet';

// Fix Leaflet's default icon path issues with Webpack
// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-expect-error
delete L.Icon.Default.prototype._getIconUrl;

L.Icon.Default.mergeOptions({
  iconRetinaUrl: markerIcon2x,
  iconUrl: markerIcon,
  shadowUrl: markerShadow
});

const tileLayerUrl = 'https://tile.openstreetmap.org/{z}/{x}/{y}.png';

function App() {
  return (
    <div className='App'>
      {/*<Header />*/}
      <MapContainer
        className='leaflet-container'
        center={[38.944444, -77.455833]}
        zoom={6}
        minZoom={3}
        maxZoom={19}
        maxBounds={[
          [-85.06, -180],
          [85.06, 180]
        ]}
        scrollWheelZoom={true}
      >
        <TileLayer url={tileLayerUrl} />
      </MapContainer>
    </div>
  );
}

export default App;
