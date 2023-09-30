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
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
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
    const { data: airportData } = await getAirports({
      bounds: {
        northEast: { lat: ne.lat, lon: ne.lng },
        southWest: { lat: sw.lat, lon: sw.lng }
      },
      limit: 100,
      page: 1
    });
    const { data: metars } = await getMetars(airportData.map((a) => a.icao));
    metars.forEach((metar) => {
      airportData.forEach((airport) => {
        if (metar.station_id == airport.icao) {
          airport.metar = metar;
        }
      });
    });
    setAirports(airportData);
  }

  function metarIcon(airport: Airport) {
    function innerIcon({ tag, color, size = 'sm' }: { tag: string; color: string; size?: string }) {
      return new DivIcon({
        html: ReactDOMServer.renderToString(
          <MantineProvider>
            <Avatar variant='filled' color={color} radius='xl' size={size}>
              {tag}
            </Avatar>
          </MantineProvider>
        ),
        className: 'metar-marker-icon'
      });
    }
    if (airport.metar?.flight_category == 'VFR') {
      return innerIcon({ tag: 'V', color: 'green' });
    } else if (airport.metar?.flight_category == 'MVFR') {
      return innerIcon({ tag: 'M', color: 'blue' });
    } else if (airport.metar?.flight_category == 'IFR') {
      return innerIcon({ tag: 'I', color: 'red' });
    } else if (airport.metar?.flight_category == 'LIFR') {
      return innerIcon({ tag: 'L', color: 'purple' });
    } else {
      return innerIcon({ tag: 'U', color: 'black', size: 'xs' });
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
