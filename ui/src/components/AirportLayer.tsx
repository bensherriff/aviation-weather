import { useState } from 'react';
import { Airport, AirportCategory } from '@lib/airport.types.ts';
import { Marker, Popup, useMapEvents } from 'react-leaflet';
import { getAirports } from '@lib/airport.ts';
import L from 'leaflet';

interface Bounds {
  northEast: { lat: number; lon: number };
  southWest: { lat: number; lon: number };
}

export default function AirportLayer() {
  const [airports, setAirports] = useState<Airport[]>([]);

  useMapEvents({
    moveend: (event) => {
      const map = event.target;
      const bounds = map.getBounds();

      const boundsParam: Bounds = {
        northEast: {
          lat: bounds.getNorth(),
          lon: bounds.getEast()
        },
        southWest: {
          lat: bounds.getSouth(),
          lon: bounds.getWest()
        }
      };

      // Call getAirports with the current map bounds and desired parameters.
      getAirports({
        bounds: boundsParam,
        metars: true,
        categories: [AirportCategory.SMALL, AirportCategory.MEDIUM, AirportCategory.LARGE],
        limit: 200
      })
        .then((response) => {
          console.log(response);
          setAirports(response.data);
        })
        .catch((error) => {
          console.error('Error fetching airports:', error);
          setAirports([]);
        });
    }
  });

  return (
    <>
      {airports.map((airport, index) => {
        const markerColor = getMarkerColor(airport);
        const icon = createCustomIcon(markerColor);
        return (
          <Marker key={index} position={[airport.latitude, airport.longitude]} icon={icon}>
            <Popup>
              <div>
                <h3>{airport.name || 'Unnamed Airport'}</h3>
                <p>ICAO: {airport.icao || 'N/A'}</p>
                <p>Flight Category: {airport.latest_metar ? airport.latest_metar.flight_category : 'No METAR Data'}</p>
              </div>
            </Popup>
          </Marker>
        );
      })}
    </>
  );
}

function getMarkerColor(airport: Airport): string {
  if (airport.latest_metar) {
    switch (airport.latest_metar.flight_category.toUpperCase()) {
      case 'IFR':
        return '#ff0100';
      case 'LIFR':
        return '#7f007f';
      case 'MVFR':
        return '#00f';
      case 'VFR':
        return '#018000';
      case 'UNKNOWN':
        return '#3e3e3e';
      default:
        return '#3e3e3e';
    }
  } else {
    return '#696969';
  }
}

function createCustomIcon(color: string): L.DivIcon {
  return L.divIcon({
    html: `<div style="
      background-color: ${color};
      width: 16px;
      height: 16px;
      border-radius: 50%;
      border: 2px solid #fff;
      "></div>`,
    className: '',
    iconSize: [20, 20],
    iconAnchor: [10, 10]
  });
}
