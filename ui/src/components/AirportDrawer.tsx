import { Divider, Drawer, Group } from '@mantine/core';
import { Airport, AirportCategory } from '@lib/airport.types.ts';

export default function AirportDrawer({
  airport,
  setAirport
}: {
  airport: Airport | null;
  setAirport: (airport: Airport | null) => void;
}) {
  if (!airport) {
    return null;
  }
  return (
    <Drawer
      opened={true}
      onClose={() => setAirport(null)}
      title={airport.name}
      withinPortal
      zIndex={10000}
      styles={{ root: { width: 0, height: 0 } }}
      padding='md'
      size='md'
      position='left'
      withOverlay={false}
      closeOnClickOutside={false}
    >
      <Group>
        <div>ICAO: {airport.icao}</div>
        <div>Category: {airportCategoryToText(airport.category)}</div>
        <div>
          Country / Region: {airport.iso_country}, {airport.iso_region}
        </div>
        <div>Municipality: {airport.municipality || 'N/A'}</div>
        <div>Local Code: {airport.local || 'N/A'}</div>
        <div>Elevation: {airport.elevation_ft}</div>
        <div>
          Coordinates: {airport.latitude.toFixed(4)}, {airport.longitude.toFixed(4)}
        </div>
        <div>Control Tower: {airport.has_tower ? 'Yes' : 'No'}</div>
        <div>Beacon: {airport.has_beacon ? 'Yes' : 'No'}</div>
        {airport.latest_metar && airport.latest_metar.flight_category && (
          <>
            <Divider my='sm' />
            <div>Flight Category: {airport.latest_metar.flight_category}</div>
          </>
        )}
      </Group>
    </Drawer>
  );
}

function airportCategoryToText(category: AirportCategory): string {
  switch (category) {
    case AirportCategory.SMALL:
      return 'Small';
    case AirportCategory.MEDIUM:
      return 'Medium';
    case AirportCategory.LARGE:
      return 'Large';
    case AirportCategory.HELIPORT:
      return 'Helipad';
    case AirportCategory.CLOSED:
      return 'Closed';
    case AirportCategory.SEAPLANE:
      return 'Seaplane Base';
    case AirportCategory.BALLOONPORT:
      return 'Balloon Port';
    default:
      return 'Unknown';
  }
}
