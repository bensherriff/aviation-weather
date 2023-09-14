'use client';

import { MapContainer } from 'react-leaflet';
import MapTiles from './MapTiles';

export default function Map({ className = '' }: { className?: string }) {
  return (
    <>
      <MapContainer
        center={[38.7209, -77.5133]}
        zoom={8}
        maxZoom={12}
        minZoom={1}
        id='map-container'
        style={{ height: '94.5vh' }}
        className={`${className} overflow-y-hidden overflow-x-hidden`}
        attributionControl={false}
      >
        <MapTiles />
      </MapContainer>
    </>
  );
}
