'use client';

import { Airport } from "@/api/airport.types";
import AirportTablePanel from "@/components/Admin/AirportTablePanel";
import CreateAirportPanel from "@/components/Admin/CreateAirportPanel";
import UpdateAirportModal from "@/components/Admin/UpdateAirportModal";
import { Container, Grid, SimpleGrid } from "@mantine/core";
import { useEffect, useState } from "react";

export default function Page() {
  const [airport, setAirport] = useState<Airport | undefined>(undefined);

  useEffect(() => {
  }, [airport]);

  return (
    <Container fluid>
      <SimpleGrid cols={{ base: 1, xs: 1 }} spacing={'md'}>
        <Grid p={'lg'}>
          <Grid.Col span={12}>
              <AirportTablePanel setAirport={setAirport} />
            </Grid.Col>
          <Grid.Col span={12}>
            <CreateAirportPanel />
          </Grid.Col>
        </Grid>
      </SimpleGrid>
      <UpdateAirportModal airport={airport} setAirport={setAirport} />
    </Container>
  );
}


