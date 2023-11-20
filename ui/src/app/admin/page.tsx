'use client';

import { Airport } from "@/api/airport.types";
import AirportTablePanel from "@/components/Admin/AirportTablePanel";
import CreateAirportPanel from "@/components/Admin/CreateAirportPanel";
import { Container, Grid } from "@mantine/core";
import { useEffect, useState } from "react";

export default function Page() {
  const [airport, setAirport] = useState<Airport | undefined>(undefined);

  useEffect(() => {
    console.log(airport);
  }, [airport]);

  return <Container fluid>
    <Grid p={'lg'}>
    <Grid.Col span={12}>
        <AirportTablePanel setAirport={setAirport} />
      </Grid.Col>
    <Grid.Col span={12}>
      <CreateAirportPanel airport={airport} setAirport={setAirport} />
    </Grid.Col>
    </Grid>
  </Container>;
}


