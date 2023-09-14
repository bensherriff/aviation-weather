'use client';

import { getAirports } from '@/js/api/airport';
import { Airport } from '@/js/api/airport.types';
import { getMetars } from '@/js/api/metar';
import { Metar } from '@/js/api/metar.types';
import { FaLocationPin } from 'react-icons/fa6';
import { DivIcon, LatLngBounds } from 'leaflet';
import { useEffect, useState } from 'react';
import ReactDOMServer from 'react-dom/server';
import { Marker, TileLayer, Tooltip, useMap, useMapEvents } from 'react-leaflet';
import MetarDialog from './MetarDialog';

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
    const _airports = await getAirports({
      bounds: {
        northEast: { lat: ne.lat, lon: ne.lng },
        southWest: { lat: sw.lat, lon: sw.lng }
      },
      limit: 100,
      page: 1
    });
    const metars = await getMetars(_airports);
    metars.forEach((metar) => {
      _airports.forEach((airport) => {
        if (metar.station_id == airport.icao) {
          airport.metar = metar;
        }
      });
    });
    setAirports(_airports);
  }

  function metarTextColor(metar: Metar | undefined) {
    if (metar?.flight_category == 'VFR') {
      return 'text-emerald-700';
    } else if (metar?.flight_category == 'MVFR') {
      return 'text-blue-700';
    } else if (metar?.flight_category == 'IFR') {
      return 'text-red-700';
    } else if (metar?.flight_category == 'LIFR') {
      return 'text-purple-700';
    } else {
      return 'text-black/50';
    }
  }

  function iconSize() {
    if (zoomLevel <= 4) {
      return 'text-xs';
    } else if (zoomLevel <= 5) {
      return 'text-sm';
    } else if (zoomLevel <= 6) {
      return 'text-base';
    } else if (zoomLevel <= 7) {
      return 'text-lg';
    } else if (zoomLevel <= 9) {
      return 'text-2xl';
    } else if (zoomLevel <= 11) {
      return 'text-3xl';
    } else if (zoomLevel >= 12) {
      return 'text-4xl';
    }
  }

  function icon(airport: Airport) {
    return new DivIcon({
      html: ReactDOMServer.renderToString(
        <FaLocationPin className={`${iconSize()} ${metarTextColor(airport.metar)}`} />
      ),
      className: 'metar-marker-icon'
    });
  }

  useEffect(() => {
    updateAirports(map.getBounds());
  }, []);

  return (
    <>
      {selectedAirport && <MetarDialog isOpen={isOpen} onClose={() => setIsOpen(false)} airport={selectedAirport} />}
      <TileLayer
        attribution='&copy; <a href="https://www.osm.org/copyright">OpenStreetMap</a> contributors'
        url='http://{s}.tile.osm.org/{z}/{x}/{y}.png'
      />
      {airports.map((airport) => (
        <Marker
          key={airport.icao}
          position={[airport.point.y, airport.point.x]}
          icon={icon(airport)}
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
