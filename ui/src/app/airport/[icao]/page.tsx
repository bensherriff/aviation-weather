'use client';

import { getAirport } from '@/api/airport';
import { Airport } from '@/api/airport.types';
import { getMetars } from '@/api/metar';
import { Metar } from '@/api/metar.types';
import { Grid, Title, Text } from '@mantine/core';
import { useEffect, useState } from 'react';

export default function Page({ params }: { params: { icao: string } }) {
  const [airport, setAirport] = useState<Airport | undefined>(undefined);
  const [metar, setMetar] = useState<Metar | undefined>(undefined);

  useEffect(() => {
    async function loadAirport() {
      const airportData = await getAirport({ icao: params.icao });
      setAirport(airportData);
      const metarData = await getMetars([airportData.icao]);
      if (metarData.length > 0) {
        setMetar(metarData[0]);
      }
    }
    loadAirport();
  }, []);

  if (airport) {
    return (
      <Grid gutter={80} style={{ margin: '0 0.5em'}}>
        <Grid.Col span={12}>
          <Title className='title' order={1}>{airport.icao} - {airport.name}</Title>
          <Text c="dimmed">
            {airport.municipality} | {airport.iso_region} | {airport.iso_country}
          </Text>
          {metar && (
            <Text c="dimmed">
              {metar.raw_text}
            </Text>
          )}
          <h3>Frequencies</h3>
          {airport.frequencies.map((frequency) => (
            <div key={frequency.frequency_mhz}>
              <ul>
                <li>{frequency.id}: {frequency.frequency_mhz} MHz</li>
              </ul>
            </div>
          ))}
          <h3>Runway Information</h3>
          {airport.runways.map((runway) => (
            <div key={runway.id}>
              <b>Runway {runway.id}</b>
              <ul>
                <li>Dimensions: {runway.length_ft} x {runway.width_ft} ft.</li>
                <li>Surface: {runway.surface}</li>
              </ul>
            </div>
          ))}
        </Grid.Col>
      </Grid>
    );
  } else {
    return <></>;
  }
}
