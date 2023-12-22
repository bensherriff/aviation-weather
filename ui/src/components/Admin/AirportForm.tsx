import { Airport, AirportCategory } from '@/api/airport.types';
import { Button, Checkbox, Container, Flex, Group, NumberInput, Paper, Select, TextInput, Title } from '@mantine/core';
import { useForm } from '@mantine/form';

interface AirportFormProps {
  title: string;
  airport?: Airport;
  submitText: string;
  onSubmit: (airport: Airport) => Promise<void>;
  onDelete?: () => Promise<void>;
}

export default function AirportForm({ title, airport, submitText, onSubmit, onDelete }: AirportFormProps) {
  const form = useForm<Airport>({
    initialValues: {
      icao: airport?.icao || '',
      category: airport?.category || AirportCategory.SMALL,
      name: airport?.name || '',
      elevation_ft: airport?.elevation_ft || 0,
      iso_country: airport?.iso_country || '',
      iso_region: airport?.iso_region || '',
      municipality:  airport?.municipality || '',
      iata: airport?.iata || '',
      local: airport?.local || '',
      latitude: airport?.latitude || 0,
      longitude: airport?.longitude || 0,
      has_tower: airport?.has_tower || false,
      has_beacon: airport?.has_beacon || false,
      runways: airport?.runways || [],
      frequencies: airport?.frequencies || [],
    }
  });

  return (
    <Container fluid>
      <Title ta='center'>{title}</Title>
      <Paper p={30} radius={'md'}>
        <form onSubmit={form.onSubmit(async (values) => {
          await onSubmit(values);
          form.reset();
        })}>
          <Group>
            <TextInput
              required
              label='ICAO'
              placeholder='KHEF'
              {...form.getInputProps('icao')}
            />
            <TextInput
              label='IATA Code'
              placeholder='HEF'
              {...form.getInputProps('iata')}
            />
            <TextInput
              label='Local Code'
              placeholder='HEF'
              {...form.getInputProps('local')}
            />
          </Group>
          <TextInput
            required
            label='Name'
            placeholder='Manassas Regional Airport/Harry P. Davis Field'
            {...form.getInputProps('name')}
          />
          <Select
            required
            label='Category'
            placeholder='Select category'
            data={[
              { value: AirportCategory.SMALL, label: 'Small' },
              { value: AirportCategory.MEDIUM, label: 'Medium' },
              { value: AirportCategory.LARGE, label: 'Large' },
              { value: AirportCategory.HELIPORT, label: 'Heliport' },
              { value: AirportCategory.CLOSED, label: 'Closed' },
              { value: AirportCategory.SEAPLANE, label: 'Seaplane Base' },
              { value: AirportCategory.BALLOONPORT, label: 'Balloonport' },
              { value: AirportCategory.UNKNOWN, label: 'Unknown'}
            ]}
            {...form.getInputProps('category')}
          />
          <Group>
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
          <Checkbox
            mt={'xs'}
            label='Has Tower'
            defaultChecked={form.values.has_tower}
            {...form.getInputProps('has_tower')}
          />
          <Checkbox
            mt={'xs'}
            label='Has Beacon'
            defaultChecked={form.values.has_beacon}
            {...form.getInputProps('has_beacon')}
          />
          <NumberInput
            required
            hideControls
            allowNegative={false}
            decimalScale={1}
            label='Elevation (ft)'
            placeholder='192.2'
            {...form.getInputProps('elevation_ft')}
          />
          <Group>
            <NumberInput
              required
              hideControls
              decimalScale={8}
              label='Latitude'
              placeholder='38.72140121'
              {...form.getInputProps('latitude')}
            />
            <NumberInput
              required
              hideControls
              decimalScale={8}
              label='Longitude'
              placeholder='-77.51540375'
              {...form.getInputProps('longitude')}
            />
          </Group>
          <Flex justify={'end'} mt={'sm'}>
            <Button type='submit'>{submitText}</Button>
            <Button color='red' ml={'sm'} onClick={() => form.reset()}>Reset</Button>
            {onDelete && (
              <Button
                variant='light'
                color='red'
                ml={'sm'}
                onClick={async () => await onDelete()}
              >
                Delete
              </Button>
            )}
          </Flex>
        </form>
      </Paper>
    </Container>
  )
}