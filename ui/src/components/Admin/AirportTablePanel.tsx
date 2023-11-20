import { getAirports, importAirports, removeAirport } from "@/api/airport";
import { Airport, AirportCategory, AirportOrderField, airportCategoryToText } from "@/api/airport.types";
import { Text, Button, Card, Group, Pagination, ScrollArea, Table, TextInput, rem, UnstyledButton, Center } from "@mantine/core";
import { HiChevronUp, HiChevronDown, HiSelector } from "react-icons/hi";
import { useEffect, useState } from "react";
import { CiSearch } from "react-icons/ci";


export default function AirportTablePanel({ setAirport }: { setAirport: (airport: Airport) => void }) {
  const [search, setSearch] = useState('');
  const [page, setPage] = useState(1);
  const [totalPages, setTotalPages] = useState(1);
  const [airports, setAirports] = useState<Airport[]>([]);

  async function getAirportData() {
    const response = await getAirports({
      page,
      limit: 100
    });
    setAirports(response.data);
    setTotalPages(response.meta.pages);
  }

  useEffect(() => {
    getAirportData();
  }, [page, search]);

  function handleSearchChange(event: any) {
    setSearch(event.currentTarget.value);
  }

  const rows = airports.map((airport) => (
    <Table.Tr
      key={airport.icao}
      onClick={() => setAirport(airport)}
      style={{ cursor: 'pointer' }}
    >
      <Table.Td>{airport.icao}</Table.Td>
      <Table.Td>{airport.full_name}</Table.Td>
      <Table.Td>{airportCategoryToText(airport.category)}</Table.Td>
      <Table.Td>{airport.continent}</Table.Td>
      <Table.Td>{airport.iso_country}</Table.Td>
      <Table.Td>{airport.iso_region}</Table.Td>
      <Table.Td>{airport.municipality}</Table.Td>
      <Table.Td>{airport.gps_code}</Table.Td>
      <Table.Td>{airport.iata_code}</Table.Td>
      <Table.Td>{airport.local_code}</Table.Td>
      <Table.Td>{airport.point.x}</Table.Td>
      <Table.Td>{airport.point.y}</Table.Td>
    </Table.Tr>
  ))

  return <Card shadow={'sm'} padding={'lg'} radius={'md'} withBorder>
    <TextInput
      placeholder="Search by ICAO"
      mb="md"
      leftSection={<CiSearch style={{ width: rem(16), height: rem(16) }} />}
      value={search}
      onChange={handleSearchChange}
    />
    <Table.ScrollContainer minWidth={500} h={500}>
      <Table highlightOnHover stickyHeader>
        <Table.Thead>
          <Table.Tr>
            <Table.Th>ICAO</Table.Th>
            <Table.Th>Full Name</Table.Th>
            <Table.Th>Category</Table.Th>
            <Table.Th>Continent</Table.Th>
            <Table.Th>ISO Country</Table.Th>
            <Table.Th>ISO Region</Table.Th>
            <Table.Th>Municipality</Table.Th>
            <Table.Th>GPS Code</Table.Th>
            <Table.Th>IATA Code</Table.Th>
            <Table.Th>Local Code</Table.Th>
            <Table.Th>Latitude</Table.Th>
            <Table.Th>Longitude</Table.Th>
          </Table.Tr>
        </Table.Thead>
        <Table.Tbody>{rows}</Table.Tbody>
      </Table>
    </Table.ScrollContainer>
    <Group>
      <Pagination value={page} total={totalPages} onChange={setPage} />
      <PanelButton onClick={async () => {
        await importAirports();
        await getAirportData();
      }}>
        Import
      </PanelButton>
      <PanelButton color={'red'} onClick={async () => {
        await removeAirport({});
        await getAirportData();
      }}>
        Remove All
      </PanelButton>
    </Group>
  </Card>
}

function PanelButton({ children, color = 'blue', onClick }: {children: any, color?: string, onClick: () => Promise<void> }) {
  const [loading, setLoading] = useState(false);
  return <Button
    loading={loading}
    variant='light'
    color={color}
    mt={'md'}
    radius={'md'}
    onClick={() => {
      setLoading(true);
      onClick().then(() => setLoading(false));
    }}
  >
    {children}
  </Button>
}

function Th({ children, asc, sorted, onSort }: { children: any, asc: boolean, sorted: boolean, onSort: () => void }) {
  const Icon = sorted ? (asc ? HiChevronUp : HiChevronDown) : HiSelector;
  return (
    <Table.Th>
      <UnstyledButton onClick={onSort}>
        <Group justify="space-between">
          <Text fw={500} fz="sm">
            {children}
          </Text>
          <Center>
            <Icon style={{ width: rem(16), height: rem(16) }} />
          </Center>
        </Group>
      </UnstyledButton>
    </Table.Th>
  );
}
