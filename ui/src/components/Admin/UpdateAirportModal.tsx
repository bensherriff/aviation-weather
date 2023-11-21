import { removeAirport, updateAirport } from "@/api/airport";
import { Airport, AirportCategory } from "@/api/airport.types";
import { Button, Container, Flex, Group, Modal, Paper, Select, TextInput, Title } from "@mantine/core";
import { useForm } from "@mantine/form";
import { useEffect } from "react";

export default function UpdateAirportModal({ airport, setAirport }: { airport: Airport | undefined, setAirport: (airport: Airport | undefined) => void}) {
  const form = useForm<Airport>({
    initialValues: {
      icao: airport?.icao || '',
      category: airport?.category || AirportCategory.SMALL,
      full_name: airport?.full_name || '',
      elevation_ft: airport?.elevation_ft || 0,
      continent: airport?.continent || '',
      iso_country: airport?.iso_country || '',
      iso_region: airport?.iso_region || '',
      municipality:  airport?.municipality || '',
      gps_code: airport?.gps_code || '',
      iata_code: airport?.iata_code || '',
      local_code: airport?.local_code || '',
      point: {
        x: airport?.point.x || 0,
        y: airport?.point.y || 0,
        srid: airport?.point.srid || 4326
      }
    }
  });

  useEffect(() => {
    if (airport) {
      form.setValues(airport);
    }
  }, [airport]);

  return (
    <Modal opened={airport !== undefined} onClose={() => setAirport(undefined)} withCloseButton={false} size={'50%'}>
      <Container>
        <Title ta='center'>Update Airport</Title>
        <Paper withBorder p={30} mt={30} radius={'md'} shadow={'sm'}>
          <form onSubmit={form.onSubmit(async (values) => {
            const response = await updateAirport({ airport: values });
            if (response.success) {
              setAirport(undefined);
            }
          })}>
            <TextInput
              required
              label='ICAO'
              placeholder='KHEF'
              {...form.getInputProps('icao')}
            />
            <Select
              required
              label='Category'
              placeholder='Select category'
              data={[
                { value: AirportCategory.SMALL, label: 'Small' },
                { value: AirportCategory.MEDIUM, label: 'Medium' },
                { value: AirportCategory.LARGE, label: 'Large' },
              ]}
              {...form.getInputProps('category')}
            />
            <TextInput
              required
              label='Full Name'
              placeholder='Manassas Regional Airport/Harry P. Davis Field'
              {...form.getInputProps('full_name')}
            />
            <TextInput
              required
              label='Elevation (ft)'
              placeholder='192'
              {...form.getInputProps('elevation_ft')}
            />
            <Group>
              <TextInput
                required
                label='Continent'
                placeholder='NA'
                {...form.getInputProps('continent')}
              />
              <TextInput
                required
                label='ISO Country'
                placeholder='US'
                {...form.getInputProps('iso_country')}
              />
              <TextInput
                required
                label='ISO Region'
                placeholder='US-VA'
                {...form.getInputProps('iso_region')}
              />
              <TextInput
                required
                label='Municipality'
                placeholder='Manassas'
                {...form.getInputProps('municipality')}
              />
            </Group>
            <Group>
              <TextInput
                required
                label='GPS Code'
                placeholder='KHEF'
                {...form.getInputProps('gps_code')}
              />
              <TextInput
                required
                label='IATA Code'
                placeholder='HEF'
                {...form.getInputProps('iata_code')}
              />
              <TextInput
                required
                label='Local Code'
                placeholder='HEF'
                {...form.getInputProps('local_code')}
              />
            </Group>
            <Group>
              <TextInput
                required
                label='Latitude'
                placeholder='38.72140121'
                {...form.getInputProps('point.x')}
              />
              <TextInput
                required
                label='Longitude'
                placeholder='-77.51540375'
                {...form.getInputProps('point.y')}
              />
            </Group>
            <Flex justify={'end'} mt={'sm'}>
              <Button type='submit' variant='light'>Update Airport</Button>
              {airport && <Button variant='light' color='red' ml={10} onClick={async () => {
                if (await removeAirport({icao: airport.icao})) {
                  setAirport(undefined);
                }
              }}>Delete</Button>}
            </Flex>
          </form>
        </Paper>
      </Container>
    </Modal>
  );
}