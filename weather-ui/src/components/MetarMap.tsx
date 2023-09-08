'use client';
import { Airport } from '@/js/airport';
import { Metar } from '@/js/weather';
import { faArrowsSpin, faLocationArrow, faLocationPin } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { DivIcon } from 'leaflet';
import Link from 'next/link';
import { useEffect, useState } from 'react';
import ReactDOMServer from 'react-dom/server';
import { MapContainer, Marker, Popup, TileLayer, Tooltip, useMapEvents } from 'react-leaflet';

export default function Map({ airportString }: { airportString: string }) {
  const [airports, setAirports] = useState<Airport[]>(JSON.parse(airportString));

  useEffect(() => {
  }, []);

  return (
    <MapContainer
      center={[38.7209, -77.5133]}
      zoom={8}
      maxZoom={12}
      minZoom={3}
      zoomControl={false}
      style={{ height: '96.5vh' }}
      className='overflow-y-hidden overflow-x-hidden'
      attributionControl={false}
    >
      <MapTiles airports={airports}/>
  </MapContainer>
  );
}

function MapTiles({ airports }: {airports: Airport[] }) {
  const [zoomLevel, setZoomLevel] = useState(8);
  // const [dragging, setDragging] = useState(false);
  // const [center, setCenter] = useState([50, 10.5]);
  const mapEvents = useMapEvents({
    zoomend: () => {
      setZoomLevel(mapEvents.getZoom());
    },
    // mouseup: () => {
    //   setCenter([mapEvents.getCenter().lat, mapEvents.getCenter().lng]);
    // }
  });

  function metarBGColor(metar: Metar | undefined) {
    if (metar?.flight_category == 'VFR') {
        return 'bg-emerald-600'
    } else if (metar?.flight_category == 'MVFR') {
        return 'bg-blue-600'
    } else if (metar?.flight_category == 'IFR') {
        return 'bg-orange-600'
    } else if (metar?.flight_category == 'LIFR') {
        return 'bg-red-600'
    } else {
        return 'bg-black'
    }
  }

  function metarTextColor(metar: Metar | undefined) {
    if (metar?.flight_category == 'VFR') {
        return 'text-emerald-700'
    } else if (metar?.flight_category == 'MVFR') {
        return 'text-blue-700'
    } else if (metar?.flight_category == 'IFR') {
        return 'text-orange-700'
    } else if (metar?.flight_category == 'LIFR') {
        return 'text-red-700'
    } else {
        return 'text-black'
    }
  }

  function windColor(metar: Metar | undefined) {
    if (Number(metar?.wind_speed_kt) <= 9) {
        return 'bg-green-300';
    } else if (Number(metar?.wind_speed_kt) > 9) {
        return 'bg-orange-300';
    } else if (Number(metar?.wind_speed_kt) > 12) {
        return 'bg-red-300';
    }
  }

  function iconSize() {
    if (zoomLevel <= 4) {
      return 'text-xs'
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
      html: ReactDOMServer.renderToString(<FontAwesomeIcon icon={faLocationPin} className={`${iconSize()} ${metarTextColor(airport.metar)}`}/>),
      className: 'metar-marker-icon'
    });
  }

  return <>
    <TileLayer
          attribution='&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
          url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
      />
      {airports.map((airport) => (
        <>
          <Marker position={[airport.latitude, airport.longitude]} icon={icon(airport)}>
          <Tooltip className='metar-tooltip' direction="top" offset={[5, -5]} opacity={1}>{airport.icao}</Tooltip>
            <Popup>
              <div className="min-w-0 flex-1 select-none">
                <Link href={`/airport/${airport.icao}`}>
                  <h1 className="text-base text-gray-900 pb-1"><span className='font-semibold'>{airport.icao}</span> {airport.name}</h1>
                </Link>
                  <hr/>
                  <p className='text-sm font-medium text-gray-500'>{airport.metar?.raw_text}</p>
                  <div className='mt-2 flex'>
                    <span className={`flex inline-block text-sm text-white ${metarBGColor(airport.metar)} py-2 px-4 rounded-full`}>{airport.metar?.flight_category? airport.metar?.flight_category : 'UNKN'}</span>
                    <div className="flex inline-block px-2">
                      <span className={`text-sm text-black ${windColor(airport.metar)} py-2 px-2 rounded-full`}>
                          {airport.metar && airport.metar.wind_dir_degrees && Number(airport.metar.wind_dir_degrees) > 0?
                              <FontAwesomeIcon className="pr-1" icon={faLocationArrow} style={{rotate: `${-45  + 180 + Number(airport.metar.wind_dir_degrees)}deg`}}/>: <></>
                          }
                          {airport.metar && airport.metar.wind_dir_degrees && airport.metar.wind_dir_degrees == 'VRB'?
                              <FontAwesomeIcon className="pr-1" icon={faArrowsSpin}/>: <></>
                          }
                          {airport.metar?.wind_speed_kt != undefined && airport.metar?.wind_speed_kt > 0? `${airport.metar?.wind_speed_kt} KT` : 'CALM'}
                      </span>
                    </div>
                  </div>
              </div>
            </Popup>
        </Marker>
        </>
      ))}
    </>;
}