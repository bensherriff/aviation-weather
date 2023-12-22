import { getAirports, importAirports, removeAirport } from "@/api/airport";
import { Airport, airportCategoryToText } from "@/api/airport.types";
import { Text, Button, Card, Group, Pagination, Table, TextInput, rem, UnstyledButton, Center, Flex, Container, Grid, Space, FileButton } from "@mantine/core";
import { HiChevronUp, HiChevronDown, HiSelector } from "react-icons/hi";
import { useEffect, useState } from "react";
import { CiSearch } from "react-icons/ci";
import { notifications } from '@mantine/notifications';


export default function AirportTablePanel({ setAirport }: { setAirport: (airport: Airport) => void }) {
  const [search, setSearch] = useState('');
  const [page, setPage] = useState(1);
  const [totalPages, setTotalPages] = useState(1);
  const [airports, setAirports] = useState<Airport[]>([]);

  async function getAirportData() {
    const response = await getAirports({
      icaos: [search],
      name: search,
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
      <Table.Td>{airport.name}</Table.Td>
      <Table.Td>{airportCategoryToText(airport.category)}</Table.Td>
      <Table.Td>{airport.iso_country}</Table.Td>
      <Table.Td>{airport.iso_region}</Table.Td>
      <Table.Td>{airport.municipality}</Table.Td>
      <Table.Td>{airport.iata}</Table.Td>
      <Table.Td>{airport.local}</Table.Td>
    </Table.Tr>
  ))

  return <Card shadow={'sm'} padding={'lg'} radius={'md'} withBorder>
    <TextInput
      placeholder="Search..."
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
            <Table.Th>Name</Table.Th>
            <Table.Th>Category</Table.Th>
            <Table.Th>ISO Country</Table.Th>
            <Table.Th>ISO Region</Table.Th>
            <Table.Th>Municipality</Table.Th>
            <Table.Th>IATA Code</Table.Th>
            <Table.Th>Local Code</Table.Th>
          </Table.Tr>
        </Table.Thead>
        <Table.Tbody>{rows}</Table.Tbody>
      </Table>
    </Table.ScrollContainer>
    <Grid mt={'md'} justify={'space-between'}>
      <Grid.Col span={10}>
        <Pagination value={page} total={totalPages} onChange={setPage} />
      </Grid.Col>
      <Grid.Col span={2}>
        <Flex justify={'end'}>
          <Space mr={'sm'}>
            <PanelFileButton accept={'.json'} onChange={async (payload) => {
              if (payload instanceof File) {
                const response = await importAirports(payload);
                if (response) {
                  await getAirportData();
                } else {
                  notifications.show({
                    title: `Failed to import airports`,
                    message: `Please try again.`,
                    color: 'red',
                    autoClose: 2000
                  });
                }
              }
            }}>
              Import
            </PanelFileButton>
          </Space>
          <Space>
            <PanelButton color={'red'} onClick={async () => {
              await removeAirport({});
              await getAirportData();
            }}>
              Remove All
            </PanelButton>
          </Space>
        </Flex>
      </Grid.Col>
    </Grid>
  </Card>
}

interface PanelButtonProps {
  children: any;
  color?: string;
  onClick?: () => Promise<void>;
}

interface PanelFileButtonProps {
  children: any;
  color?: string;
  multiple?: boolean;
  accept?: string;
  onChange?: (payload: File|File[]|null) => Promise<void>;
}

function PanelFileButton({ children, multiple = false, accept, color, onChange = async () => {} }: PanelFileButtonProps) {
  const [loading, setLoading] = useState(false);
  return <FileButton
    multiple={multiple}
    accept={accept}
    onChange={(e) => {
      setLoading(true);
      onChange(e).then(() => setLoading(false));
    }}
  >
    {(props) => <Button loading={loading} variant='light' color={color} radius='md' {...props}>{children}</Button>}
  </FileButton>
}

function PanelButton({ children, color = 'blue', onClick = async () => {} }: PanelButtonProps) {
  const [loading, setLoading] = useState(false);
  return <Button
    loading={loading}
    variant='light'
    color={color}
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
