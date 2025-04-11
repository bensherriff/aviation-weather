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
      categories: [AirportCategory.HELIPORT, AirportCategory.SMALL, AirportCategory.MEDIUM, AirportCategory.LARGE]
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

  const categoryOrder: { [key in AirportCategory]?: number } = {
    [AirportCategory.LARGE]: 3,
    [AirportCategory.MEDIUM]: 2,
    [AirportCategory.SMALL]: 1,
    [AirportCategory.HELIPORT]: 0
  };

  const sortedAirports = airports.slice().sort((a, b) => {
    // Compare by airport category first.
    const categoryA = categoryOrder[a.category] ?? 4;
    const categoryB = categoryOrder[b.category] ?? 4;
    if (categoryA !== categoryB) {
      return categoryA - categoryB;
    }

    // Then compare by flight category if available.
    // Assuming that latest_metar.flight_category is a string and "UNKN" needs to come last.
    const fcA = a.latest_metar?.flight_category ?? 'UNKN';
    const fcB = b.latest_metar?.flight_category ?? 'UNKN';

    if (fcA === 'UNKN' && fcB !== 'UNKN') return 1;
    if (fcB === 'UNKN' && fcA !== 'UNKN') return -1;

    // If both flight categories are not "UNKN", do a simple alphabetical comparison.
    // (You may wish to customize this logic based on the actual flight category values.)
    if (fcA < fcB) return -1;
    if (fcA > fcB) return 1;
    return 0;
  });

  return (
    <>
      {sortedAirports.map((airport, index) => (
        <AirportMarker key={index} airport={airport} index={index} setAirport={setAirport} />
      ))}
    </>
  );
}
