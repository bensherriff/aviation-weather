'use client';

import { getAirports, updateAirport } from '@/api/airport';
import { Airport, AirportOrderField } from '@/api/airport.types';
import { getMetars } from '@/api/metar';
import { LatLngBounds, icon } from 'leaflet';
import { useEffect, useState } from 'react';
import { Marker, TileLayer, Tooltip, useMap, useMapEvents } from 'react-leaflet';
import MetarModal from './MetarModal';
import { useRecoilState, useRecoilValue } from 'recoil';
import { coordinatesState, zoomState } from '@/state/map';

export default function MapTiles() {
  const [isOpen, setIsOpen] = useState(false);
  const [airports, setAirports] = useState<Airport[]>([]);
  const [selectedAirport, setSelectedAirport] = useState<Airport | undefined>();
  const coordinates = useRecoilValue(coordinatesState);
  const [zoom, setZoom] = useRecoilState(zoomState);
  // const [dragging, setDragging] = useState(false);
  const map = useMap();

  const mapEvents = useMapEvents({
    zoomend: async () => {
      setZoom(mapEvents.getZoom());
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

  useEffect(() => {
    map.setView([coordinates.lat, coordinates.lon]);
  }, [coordinates]);

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
      categories: ['large_airport', 'medium_airport', 'small_airport'],
      order_field: AirportOrderField.CATEGORY,
      order_by: 'asc',
      limit: zoom < 4 ? 200 : 100,
      page: 1
    });
    const airports = airportData.filter((airport) => airport.has_metar);
    const { data: metars } = await getMetars(airports.map((a) => a.icao));
    metars.forEach((metar) => {
      airportData.forEach((airport) => {
        if (metar.station_id == airport.icao) {
          airport.latest_metar = metar;
        }
      });
    });
    setAirports(airportData);
  }

  function metarIcon(airport: Airport) {
    let iconUrl = '/icons/unkn.svg';
    if (!airport.has_metar && airport.latest_metar == undefined) {
      iconUrl = '/icons/nometar.svg';
    } else if (airport.latest_metar?.flight_category == 'VFR') {
      iconUrl = '/icons/vfr.svg';
    }  else if (airport.latest_metar?.flight_category == 'MVFR') {
      iconUrl = '/icons/mvfr.svg';
    } else if (airport.latest_metar?.flight_category == 'IFR') {
      iconUrl = '/icons/ifr.svg';
    } else if (airport.latest_metar?.flight_category == 'LIFR') {
      iconUrl = '/icons/lifr.svg';
    }
    return icon({
      iconUrl: iconUrl,
      iconSize: [20, 20]
    })
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
          position={[airport.latitude, airport.longitude]}
          icon={metarIcon(airport)}
          eventHandlers={{
            click: () => handleOpen(airport)
          }}
        >
          {!isOpen && (
            <Tooltip className='metar-tooltip' direction='top' offset={[5, -5]} opacity={1}>
              <b>{airport.icao}</b> - {airport.name}
            </Tooltip>
          )}
        </Marker>
      ))}
    </>
  );
}
