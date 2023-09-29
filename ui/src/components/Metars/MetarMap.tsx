'use client';

import { MapContainer } from 'react-leaflet';
import MapTiles from './MapTiles';
import './metars.css';

export default function Map() {
  return (
    <>
      <MapContainer
        center={[38.7209, -77.5133]}
        zoom={8}
        maxZoom={14} // Zoomed in
        minZoom={3} // Zoomed out
        id='map-container'
        style={{ height: '94.5vh' }}
        className={`overflow-y-hidden overflow-x-hidden`}
        attributionControl={false}
      >
        <MapTiles />
      </MapContainer>
    </>
  );
}
