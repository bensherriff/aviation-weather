'use client';

import { Airport } from "@/api/airport.types";
import AirportTablePanel from "@/components/Admin/AirportTablePanel";
import CreateAirportPanel from "@/components/Admin/CreateAirportPanel";
import UpdateAirportModal from "@/components/Admin/UpdateAirportModal";
import { isAdminState } from "@/state/auth";
import { Container, Grid, SimpleGrid } from "@mantine/core";
import { useState } from "react";
import { useRecoilValue } from "recoil";

export default function Page() {
  const [airport, setAirport] = useState<Airport | undefined>(undefined);
  const isAdmin = useRecoilValue(isAdminState);

  return (
    <>
      {isAdmin && (
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
      )}
    </>
  );
}


