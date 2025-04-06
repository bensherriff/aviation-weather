'use client';

import { MapContainer } from 'react-leaflet';
import MapTiles from './MapTiles';
import './metars.css';
import { coordinatesState, zoomState } from '@/state/map';
import { useRecoilValue } from 'recoil';

export default function Map() {
  const coordinates = useRecoilValue(coordinatesState);
  const zoom = useRecoilValue(zoomState);

  return (
    <>
      <MapContainer
        center={[coordinates.lat, coordinates.lon]}
        zoom={zoom}
        maxZoom={14} // Zoomed in
        minZoom={3} // Zoomed out
        id='map-container'
        className={`map-container`}
        attributionControl={false}
      >
        <MapTiles />
      </MapContainer>
    </>
  );
}
