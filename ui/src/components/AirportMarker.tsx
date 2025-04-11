import { Airport } from '@lib/airport.types.ts';
import { Marker } from 'react-leaflet';
import L from 'leaflet';

export default function AirportMarker({
  index,
  airport,
  setAirport
}: {
  index: number;
  airport: Airport;
  setAirport: (airport: Airport) => void;
}) {
  const markerColor = getMarkerColor(airport);
  const icon = createCustomIcon(markerColor);
  return (
    <Marker
      key={index}
      position={[airport.latitude, airport.longitude]}
      icon={icon}
      eventHandlers={{
        click: () => setAirport(airport)
      }}
    />
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
