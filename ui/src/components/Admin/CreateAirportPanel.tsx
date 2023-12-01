import { createAirport } from "@/api/airport";
import { Airport, AirportCategory } from "@/api/airport.types";
import { Card, TextInput, Select, Group, Flex, Space, Button } from "@mantine/core";
import { useForm } from "@mantine/form";

export default function CreateAirportPanel() {
  const form = useForm<Airport>({
    initialValues: {
      icao: '',
      category: AirportCategory.SMALL,
      full_name: '',
      elevation_ft: 0,
      iso_country: '',
      iso_region: '',
      municipality: '',
      gps_code: '',
      iata_code: '',
      local_code: '',
      point: {
        x: 0,
        y: 0,
        srid: 4326
      }
    }
  });

  return <Card shadow={'sm'} padding={'lg'} radius={'md'} withBorder>
    Create Airport
    <form onSubmit={form.onSubmit(async (values) => {
      const response = await createAirport({ airport: values });
      if (response.success) {
        form.reset();
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
          label='IATA Code'
          placeholder='MNZ'
          {...form.getInputProps('iata_code')}
        />
        <TextInput
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
        <Space mr={'sm'}>
          <Button
            type='submit'
            variant='light'
            color='blue'
            radius={'md'}
          >
            Create
          </Button>
        </Space>
        <Space>
          <Button
            type='button'
            variant='light'
            color='red'
            radius={'md'}
            onClick={() => form.reset()}
          >
            Reset
          </Button>
        </Space>
      </Flex>
    </form>
  </Card>
}