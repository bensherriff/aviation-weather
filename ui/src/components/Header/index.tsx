'use client';

import Link from 'next/link';
import { useEffect, useState } from 'react';
import { getAirport, getAirports } from '@/api/airport';
import { Autocomplete, Avatar, Button, Card, FileButton, Grid, Group, Menu, Text, UnstyledButton } from '@mantine/core';
import './header.css';
import { refresh, refreshLoggedIn, logout } from '@/api/auth';
import Cookies from 'js-cookie';
import { useRecoilState } from 'recoil';
import { userState } from '@/state/auth';
import { getFavorites, getPicture, setPicture } from '@/api/users';
import { useToggle } from '@mantine/hooks';
import { HeaderModal } from './HeaderModal';
import { favoritesState } from '@/state/user';
import { coordinatesState, zoomState } from '@/state/map';

export default function Header() {
  const [loading, setLoading] = useState(true);
  const [searchValue, setSearchValue] = useState('');
  const [airports, setAirports] = useState<{ key: string; value: string; label: string }[]>([]);
  const [modalType, toggle] = useToggle([undefined, 'login', 'register', 'reset']);
  const [user, setUser] = useRecoilState(userState);
  const [favorites, setFavorites] = useRecoilState(favoritesState);
  const [refreshId, setRefreshId] = useState<NodeJS.Timeout | undefined>(undefined);
  const [profilePicture, setProfilePicture] = useState<File | null>(null);
  const [coordinates, setCoordinates] = useRecoilState(coordinatesState);
  const [zoom, setZoom] = useRecoilState(zoomState);

  useEffect(() => {
    if (!user || !Cookies.get('logged_in')) {
      refresh().then((response) => {
        if (response) {
          setRefreshId(refreshLoggedIn());
          setUser(response.user);
          getFavorites().then((response) => {
            if (response) {
              setFavorites(response);
            }
          });
          if (response.user.profile_picture) {
            getPicture().then((response) => {
              if (response) {
                setProfilePicture(response as File);
              }
            });
          }
        }
      });
    }
    setLoading(false);
  }, [user]);

  async function onChange(value: string) {
    setSearchValue(value);
    const airportData = await getAirports({ search: value });
    setAirports(
      airportData.data.map((airport) => ({
        key: airport.icao,
        value: airport.icao,
        label: `${airport.icao} - ${airport.name}`
      }))
    );
  }

  async function onClick(value: string) {
    const airport = await getAirport({ icao: value });
    if (airport) {
      setCoordinates({ lat: airport.data.latitude, lon: airport.data.longitude });
    }
  }

  return (
    <>
      <nav className='navbar'>
        <div className='left'>
          <Link href={'/'} className='title'>
            <span>Aviation Weather</span>
          </Link>
          <div className='search'>
            <Autocomplete
              radius='xl'
              placeholder='Search Airports...'
              data={airports}
              limit={10}
              value={searchValue}
              onChange={onChange}
              onOptionSubmit={onClick}
              onBlur={() => setSearchValue('')}
            />
          </div>
        </div>
        <UserSection
          profilePicture={profilePicture}
          setProfilePicture={setProfilePicture}
          toggle={toggle}
          setFavorites={setFavorites}
          refreshId={refreshId}
          loading={loading}
        />
      </nav>
      <HeaderModal
        type={modalType}
        toggle={toggle}
        setUser={(u) => {
          setUser(u);
          getFavorites().then((response) => {
            if (response) {
              setFavorites(response);
            }
          });
          if (u.profile_picture) {
            getPicture().then((response) => {
              if (response) {
                setProfilePicture(response as File);
              }
            });
          }
        }}
        setRefreshId={setRefreshId}
      />
    </>
  );
}

interface UserSectionProps {
  profilePicture: File | null;
  setProfilePicture: (picture: File | null) => void;
  toggle: (type: string) => void;
  setFavorites: (favorites: string[]) => void;
  refreshId: NodeJS.Timeout | undefined;
  loading: boolean;
}

function UserSection({ profilePicture, setProfilePicture, setFavorites, refreshId, toggle, loading }: UserSectionProps) {
  const [user, setUser] = useRecoilState(userState);

  return (
    <div className='user-section'>
      {loading ? (
        <></>
      ) : (
        <>
          {user ? (
            <Menu shadow='md' width={200} openDelay={100} closeDelay={400}>
              <Menu.Target>
                <UnstyledButton className='user user-button'>
                  <Group>
                    <Avatar src={profilePicture ? URL.createObjectURL(profilePicture) : undefined} />
                    <div style={{ flex: 1 }}>
                      <Text size='sm' fw={500}>
                        {user.first_name} {user.last_name}
                      </Text>
                      <Text c='dimmed' size='xs' style={{ textTransform: 'uppercase' }}>
                        {user.role}
                      </Text>
                    </div>
                  </Group>
                </UnstyledButton>
              </Menu.Target>
              <Menu.Dropdown p={0}>
                <Card>
                  <Card.Section h={140} style={{ backgroundColor: '#4481e3' }} />
                  <FileButton
                    onChange={(payload) => {
                      if (payload) {
                        setPicture(payload).then((response) => {
                          if (response) {
                            setProfilePicture(payload);
                          }
                        });
                      }
                    }}
                    accept='image/png,image/jpeg,image/jpg'
                    multiple={false}
                  >
                    {(props) => (
                      <Avatar
                        {...props}
                        component='button'
                        size={80}
                        radius={80}
                        mx={'auto'}
                        mt={-30}
                        style={{ cursor: 'pointer' }}
                        bg={profilePicture ? 'transparent' : 'white'}
                        src={profilePicture ? URL.createObjectURL(profilePicture) : undefined}
                      />
                    )}
                  </FileButton>
                  <Text ta='center' fz='lg' fw={500} mt='sm'>
                    {user.first_name} {user.last_name}
                  </Text>
                  <Text ta='center' fz='sm' c='dimmed' style={{ textTransform: 'uppercase' }}>
                    {user.role}
                  </Text>
                  <Grid mt='xl'>
                    <Grid.Col span={6}>
                      <Link href='/profile'>
                        <Button fullWidth radius='md' size='xs' variant='default'>
                          Profile
                        </Button>
                      </Link>
                    </Grid.Col>
                    <Grid.Col span={6}>
                      <Button
                        fullWidth
                        radius='md'
                        size='xs'
                        variant='default'
                        onClick={async () => {
                          await logout();
                          Cookies.remove('logged_in');
                          setUser(undefined);
                          setFavorites([]);
                          setProfilePicture(null);
                          if (refreshId) {
                            clearInterval(refreshId);
                          }
                        }}
                      >
                        Logout
                      </Button>
                    </Grid.Col>
                    {user.role == 'admin' && (
                      <Grid.Col span={12}>
                        <Link href='/admin'>
                          <Button fullWidth radius='md' size='xs' variant='default'>
                            Administration
                          </Button>
                        </Link>
                      </Grid.Col>
                    )}
                  </Grid>
                </Card>
              </Menu.Dropdown>
            </Menu>
          ) : (
            <Group className='user'>
              <Button onClick={() => toggle('login')}>Login</Button>
              <Button variant='outline' onClick={() => toggle('register')}>
                Sign up
              </Button>
            </Group>
          )}
        </>
      )}
    </div>
  )
}
