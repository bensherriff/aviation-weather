'use client';

import { getAirports } from '@/api/airport';
import { Airport } from '@/api/airport.types';
import { getMetars } from '@/api/metar';
import { DivIcon, LatLngBounds } from 'leaflet';
import { useEffect, useState } from 'react';
import ReactDOMServer from 'react-dom/server';
import { Marker, TileLayer, Tooltip, useMap, useMapEvents } from 'react-leaflet';
import MetarModal from './MetarModal';
import { BsCircle, BsCircleFill } from 'react-icons/bs';

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

  function metarIcon(airport: Airport) {
    if (airport.metar?.flight_category == 'VFR') {
      return new DivIcon({
        html: ReactDOMServer.renderToString(
          <div>
            <BsCircle className={`${iconSize()} rounded-full bg-emerald-700`} />
            <span className={`${iconSize()} text-white`}>V</span>
          </div>
        ),
        className: 'metar-marker-icon'
      });
    } else if (airport.metar?.flight_category == 'MVFR') {
      return new DivIcon({
        html: ReactDOMServer.renderToString(
          <div>
            <BsCircle className={`${iconSize()} rounded-full bg-blue-700`} />
            <span className={`${iconSize()} text-white`}>M</span>
          </div>
        ),
        className: 'metar-marker-icon'
      });
    } else if (airport.metar?.flight_category == 'IFR') {
      return new DivIcon({
        html: ReactDOMServer.renderToString(
          <div>
            <BsCircle className={`${iconSize()} rounded-full bg-red-700`} />
            <span className={`${iconSize()} text-white`}>I</span>
          </div>
        ),
        className: 'metar-marker-icon'
      });
    } else if (airport.metar?.flight_category == 'LIFR') {
      return new DivIcon({
        html: ReactDOMServer.renderToString(
          <div>
            <BsCircle className={`${iconSize()} rounded-full bg-purple-700`} />
            <span className={`${iconSize()} text-white`}>L</span>
          </div>
        ),
        className: 'metar-marker-icon'
      });
    } else {
      return new DivIcon({
        html: ReactDOMServer.renderToString(<BsCircleFill className={`text-black`} />),
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
