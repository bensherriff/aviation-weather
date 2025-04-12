import { Airport, AirportCategory } from '@lib/airport.types.ts';
import { Marker, Popup } from 'react-leaflet';
import L from 'leaflet';
import { useRef } from 'react';

export default function AirportMarker({
  index,
  airport,
  setAirport
}: {
  index: number;
  airport: Airport;
  setAirport: (airport: Airport) => void;
}) {
  const icon = createCustomIcon(airport);
  const markerRef = useRef<L.Marker>(null);

  return (
    <Marker
      key={index}
      ref={markerRef}
      position={[airport.latitude, airport.longitude]}
      icon={icon}
      eventHandlers={{
        click: () => setAirport(airport),
        mouseover: () => markerRef.current?.openPopup(),
        mouseout: () => markerRef.current?.closePopup()
      }}
    >
      <Popup closeButton={false} autoPan={false}>
        {airport.icao} - {airport.name}
      </Popup>
    </Marker>
  );
}

function getMarkerColor(flightCategory: 'VFR' | 'MVFR' | 'LIFR' | 'IFR' | 'UNKN'): string {
  switch (flightCategory) {
    case 'IFR':
      return '#ff0100';
    case 'LIFR':
      return '#7f007f';
    case 'MVFR':
      return '#00f';
    case 'VFR':
      return '#018000';
    case 'UNKN':
      return '#696969';
  }
}

function createCustomIcon(airport: Airport): L.DivIcon {
  if (airport.category === AirportCategory.HELIPORT) {
    return L.divIcon({
      html: `
        <div style="
          width: 14px;
          height: 14px;
          border-radius: 50%;
          border: 2px solid black;
          background-color: white;
          display: flex;
          align-items: center;
          justify-content: center;">
          <span style="color: black; font-size: 8px; font-weight: bold;">H</span>
        </div>
      `,
      className: '',
      iconSize: [20, 20],
      iconAnchor: [10, 10]
    });
  } else {
    // Default to a filled circle.
    const flightCategory = airport.latest_metar?.flight_category || 'UNKN';
    const color = getMarkerColor(flightCategory);
    if (flightCategory == 'UNKN') {
      return L.divIcon({
        html: `
        <div style="
          background-color: ${color};
          width: 10px;
          height: 10px;
          border-radius: 50%;">
        </div>
      `,
        className: '',
        iconSize: [20, 20],
        iconAnchor: [10, 10]
      });
    } else {
      return L.divIcon({
        html: `
        <div style="
          background-color: ${color};
          width: 18px;
          height: 18px;
          border-radius: 50%;
          border: 2px solid #fff;">
        </div>
      `,
        className: '',
        iconSize: [20, 20],
        iconAnchor: [10, 10]
      });
    }
  }
}
