'use client';

import { getAirports } from '@/api/airport';
import { Airport } from '@/api/airport.types';
import { getMetars } from '@/api/metar';
import { DivIcon, LatLngBounds } from 'leaflet';
import { useEffect, useState } from 'react';
import ReactDOMServer from 'react-dom/server';
import { Marker, TileLayer, Tooltip, useMap, useMapEvents } from 'react-leaflet';
import MetarModal from './MetarModal';
import { Avatar, MantineProvider } from '@mantine/core';

export default function MapTiles() {
  const [isOpen, setIsOpen] = useState(false);
  const [airports, setAirports] = useState<Airport[]>([]);
  const [selectedAirport, setSelectedAirport] = useState<Airport | undefined>();
  const [zoomLevel, setZoomLevel] = useState(8);
  // const [dragging, setDragging] = useState(false);
  const map = useMap();

  const mapEvents = useMapEvents({
    zoomend: async () => {
      setZoomLevel(mapEvents.getZoom());
      await updateAirports(mapEvents.getBounds());
    },
    movestart: () => {
      // setDragging(true);
    },
    moveend: async () => {
      // setDragging(false);
      await updateAirports(mapEvents.getBounds());
    }
  });

  function handleOpen(airport: Airport) {
    setSelectedAirport(airport);
    setIsOpen(true);
  }

  async function updateAirports(bounds: LatLngBounds) {
    const ne = bounds.getNorthEast();
    const sw = bounds.getSouthWest();
    const { data: _airports } = await getAirports({
      bounds: {
        northEast: { lat: ne.lat, lon: ne.lng },
        southWest: { lat: sw.lat, lon: sw.lng }
      },
      limit: 100,
      page: 1
    });
    const { data: metars } = await getMetars(_airports);
    metars.forEach((metar) => {
      _airports.forEach((airport) => {
        if (metar.station_id == airport.icao) {
          airport.metar = metar;
        }
      });
    });
    setAirports(_airports);
  }

  function iconSize() {
    if (zoomLevel <= 5) {
      return 'xs';
    } else {
      return 'sm';
    }
  }

  function metarIcon(airport: Airport) {
    if (airport.metar?.flight_category == 'VFR') {
      return new DivIcon({
        html: ReactDOMServer.renderToString(
          <MantineProvider>
            <Avatar variant='filled' color='green' radius='xl' size={iconSize()}>
              V
            </Avatar>
          </MantineProvider>
        ),
        className: 'metar-marker-icon'
      });
    } else if (airport.metar?.flight_category == 'MVFR') {
      return new DivIcon({
        html: ReactDOMServer.renderToString(
          <MantineProvider>
            <Avatar variant='filled' color='blue' radius='xl' size={iconSize()}>
              M
            </Avatar>
          </MantineProvider>
        ),
        className: 'metar-marker-icon'
      });
    } else if (airport.metar?.flight_category == 'IFR') {
      return new DivIcon({
        html: ReactDOMServer.renderToString(
          <MantineProvider>
            <Avatar variant='filled' color='red' radius='xl' size={iconSize()}>
              I
            </Avatar>
          </MantineProvider>
        ),
        className: 'metar-marker-icon'
      });
    } else if (airport.metar?.flight_category == 'LIFR') {
      return new DivIcon({
        html: ReactDOMServer.renderToString(
          <MantineProvider>
            <Avatar variant='filled' color='purple' radius='xl' size={iconSize()}>
              L
            </Avatar>
          </MantineProvider>
        ),
        className: 'metar-marker-icon'
      });
    } else {
      return new DivIcon({
        html: ReactDOMServer.renderToString(
          <MantineProvider>
            <Avatar variant='filled' color='black' radius='xl' size={iconSize()}>
              U
            </Avatar>
          </MantineProvider>
        ),
        className: 'metar-marker-icon'
      });
    }
  }

  useEffect(() => {
    updateAirports(map.getBounds());
  }, []);

  return (
    <>
      {selectedAirport && <MetarModal isOpen={isOpen} onClose={() => setIsOpen(false)} airport={selectedAirport} />}
      <TileLayer
        attribution='&copy; <a href="https://www.osm.org/copyright">OpenStreetMap</a> contributors'
        url='http://{s}.tile.osm.org/{z}/{x}/{y}.png'
      />
      {airports.map((airport) => (
        <Marker
          key={airport.icao}
          position={[airport.point.y, airport.point.x]}
          icon={metarIcon(airport)}
          eventHandlers={{
            click: () => handleOpen(airport)
          }}
        >
          {!isOpen && (
            <Tooltip className='metar-tooltip' direction='top' offset={[5, -5]} opacity={1}>
              <b>{airport.icao}</b> - {airport.full_name}
            </Tooltip>
          )}
        </Marker>
      ))}
    </>
  );
}
