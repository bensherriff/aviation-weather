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
      const { data: airportData } = await getAirport({ icao: params.icao });
      setAirport(airportData);
      const { data: metarData } = await getMetars([airportData.icao]);
      if (metarData.length > 0) {
        setMetar(metarData[0]);
      }
    }
    loadAirport();
  }, []);

  if (airport) {
    return (
      <Grid gutter={80} style={{ margin: '1em auto 0'}}>
        <Grid.Col span={12}>
          <Title className='title' order={1}>{airport.icao} - {airport.name}</Title>
          <Text c="dimmed">
            {airport.municipality} | {airport.iso_region} | {airport.iso_country}
          </Text>
        </Grid.Col>
      </Grid>
    );
  } else {
    return <></>;
  }
}
