'use client';

import { createAirport, removeAirport, updateAirport } from "@/api/airport";
import { Airport } from "@/api/airport.types";
import AirportForm from "@/components/Admin/AirportForm";
import AirportTablePanel from "@/components/Admin/AirportTablePanel";
import { isAdminState } from "@/state/auth";
import { Container, Grid, Modal, SimpleGrid } from "@mantine/core";
import { useState } from "react";
import { useRecoilValue } from "recoil";

export default function Page() {
  const [showModal, setShowModal] = useState(false);
  const [airport, setAirport] = useState<Airport | undefined>(undefined);
  const isAdmin = useRecoilValue(isAdminState);

  return (
    <>
      {isAdmin && (
        <Container fluid>
        <SimpleGrid cols={{ base: 1, xs: 1 }} spacing={'md'}>
          <Grid p={'lg'}>
            <Grid.Col span={12}>
                <AirportTablePanel setShowModal={setShowModal} setAirport={setAirport} />
              </Grid.Col>
          </Grid>
        </SimpleGrid>
        <Modal size={'xl'} opened={showModal} onClose={() => {
          setAirport(undefined);
          setShowModal(false);
        }}>
          <AirportForm
            title={airport ? 'Update Airport' : 'Create Airport'}
            submitText={airport ? 'Update' : 'Create'}
            airport={airport}
            onDelete={airport ? async () => {
              const response = await removeAirport({ icao: airport.icao });
              setShowModal(false);
            } : undefined}
            onSubmit={async (value) => {
              if (airport) {
                const response = await updateAirport({ airport: value });
              } else {
                const response = await createAirport({ airport: value });
              }
              setShowModal(false);
            }}
          />
        </Modal>
      </Container>
      )}
    </>
  );
}


