'use client';

import { getAirports } from "@/api/airport";
import { Airport } from "@/api/airport.types";
import { useEffect, useState } from "react";
import { useRecoilState, useRecoilValue } from "recoil";
import { Badge, Button, Card, Grid, Group, SimpleGrid, Text, Title } from "@mantine/core";
import classes from './profile.module.css';
import { getFavorites, removeFavorite } from "@/api/users";
import { getMetars } from "@/api/metar";
import { Metar } from "@/api/metar.types";
import { MdLocationSearching } from 'react-icons/md';
import { useRouter } from "next/navigation";
import { coordinatesState } from "@/state/map";
import { userState } from "@/state/auth";

export default function Page() {
  const user = useRecoilValue(userState);
  const router = useRouter();

  useEffect(() => {
    if (!user) {
      router.push('/');
    }
  }, []);
  
  return (
    <Grid gutter={80}>
      <Grid.Col span={12}>
        <Card shadow="sm" m={"lg"} padding="lg" radius="md" withBorder>
          <Title className={classes.title} order={2}>
            {user?.first_name} {user?.last_name}
          </Title>
          <hr />
          <Text c="dimmed">
            
          </Text>
        </Card>
      </Grid.Col>
      <Grid.Col span={12}>
        <TopSection />
      </Grid.Col>
    </Grid>
  );
}

function TopSection() {
  const [airports, setAirports] = useState<Airport[]>([]);
  const [metars, setMetars] = useState<Metar[]>([]);
  const router = useRouter();
  const [_, setCoordinates] = useRecoilState(coordinatesState);

  useEffect(() => {
    updateFavorites();
  }, []);

  function metarColor(metar?: Metar): string {
    switch (metar?.flight_category) {
      case 'VFR':
        return 'green';
      case 'MVFR':
        return 'blue';
      case 'IFR':
        return 'red';
      case 'LIFR':
        return 'purple';
      default:
        return 'gray';
    }
  }

  function AirportCard(airport: Airport) {
    let metar = metars.find((m) => m.station_id === airport.icao);
    let color = metarColor(metar);
    let text = metar?.flight_category || 'UNKN';

    return (
      <Card key={airport.icao} shadow="sm" padding="lg" radius="md" withBorder>
        <Group justify="space-between" mt="md" mb="xs">
          <Text fw={500} style={{ textOverflow: 'ellipsis', overflow: 'hidden', whiteSpace: 'nowrap', width: '20em' }}>{airport.name}</Text>
          <Badge color={color} variant="light">{text}</Badge>
        </Group>
        <Group style={{ cursor: 'pointer', userSelect: 'none' }} onClick={() => {
          setCoordinates({
            lat: airport.latitude,
            lon: airport.longitude,
          });
          router.push('/');
        }}>
          <MdLocationSearching size={20} />
          <Text size="sm" c="dimmed">
            {airport.latitude.toFixed(3)}, {airport.longitude.toFixed(3)}
          </Text>
        </Group>
        <Group style={{
          display: 'flex',
          justifyContent: 'end',
          alignItems: 'center',
        }}>
          <Button
            variant="outline"
            color="blue"
            size="sm"
            radius="lg"
            style={{ marginTop: '10px' }}
          >
            View
          </Button>
          <Button
            variant="outline"
            color="red"
            size="sm"
            radius="lg"
            style={{ marginTop: '10px' }}
            onClick={async () => {
              await removeFavorite(airport.icao);
              await updateFavorites();
            }}
          >
            Remove
          </Button>
        </Group>
      </Card>
    );
  }

  async function updateFavorites() {
    const favorites = await getFavorites();
    const m = (await getMetars(favorites)).data;
    setMetars(m);
    const a = (await getAirports({ icaos: favorites })).data;
    setAirports(a);
  }

  return (
    <div className={classes.wrapper}>
      <Grid gutter={80}>
        <Grid.Col span={{ base: 12, md: 5 }}>
          <Title className={classes.title} order={2}>
            Logbook
          </Title>
          <hr />
          <Text c="dimmed">
            Your logbook is a list of your flights. You can add flights to your logbook by clicking the "Add to logbook" button on the flight page.
          </Text>
        </Grid.Col>
        <Grid.Col span={{ base: 12, md: 7 }}>
          <Title className={classes.title} order={2}>
            Saved Airports
          </Title>
          <hr />
          <SimpleGrid cols={{ base: 1, md: 2 }} spacing={30}>
            {airports.map((airport) => AirportCard(airport))}
          </SimpleGrid>
        </Grid.Col>
      </Grid>
  </div>
  );
}
