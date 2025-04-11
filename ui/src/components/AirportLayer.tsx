import { useEffect, useState } from 'react';
import { Airport, AirportCategory } from '@lib/airport.types.ts';
import { useMapEvents } from 'react-leaflet';
import { getAirports } from '@lib/airport.ts';
import AirportMarker from '@components/AirportMarker.tsx';
import { LeafletEvent } from 'leaflet';

interface Bounds {
  northEast: { lat: number; lon: number };
  southWest: { lat: number; lon: number };
}

export default function AirportLayer({ setAirport }: { setAirport: (airport: Airport) => void }) {
  const [airports, setAirports] = useState<Airport[]>([]);

  function loadAirports(event: LeafletEvent) {
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

    getAirports({
      bounds: boundsParam,
      metars: true,
      categories: [AirportCategory.SMALL, AirportCategory.MEDIUM, AirportCategory.LARGE],
      limit: 200
    })
      .then((response) => {
        setAirports(response.data);
      })
      .catch((error) => {
        console.error('Error fetching airports:', error);
        setAirports([]);
      });
  }

  const map = useMapEvents({
    moveend: loadAirports
  });

  useEffect(() => {
    if (map) {
      loadAirports({ target: map } as LeafletEvent);
    }
  }, [map]);

  return (
    <>
      {airports.map((airport, index) => {
        return <AirportMarker airport={airport} index={index} setAirport={setAirport} />;
      })}
    </>
  );
}
