'use client';

import Link from 'next/link';
import { useState } from 'react';
import { getAirport, getAirports } from '@/api/airport';
import { Autocomplete, Avatar, Button, Card, FileButton, Grid, Group, Menu, Text, UnstyledButton } from '@mantine/core';
import './header.css';
import { SetterOrUpdater, useRecoilState } from 'recoil';
import { setPicture } from '@/api/users';
import { useToggle } from '@mantine/hooks';
import { HeaderModal } from './HeaderModal';
import { coordinatesState } from '@/state/map';
import { User } from '@/api/auth.types';
import { usePathname, useRouter } from 'next/navigation';

interface HeaderProps {
  user: User | undefined;
  profilePicture: File | undefined;
  setProfilePicture: SetterOrUpdater<File | undefined>;
  login: ({ email, password }: { email: string, password: string }) => Promise<boolean>;
  logout: () => Promise<void>;
  register: ({ firstName, lastName, email, password }: { firstName: string, lastName: string, email: string, password: string }) => Promise<boolean>;
}

export default function Header({ user, profilePicture, setProfilePicture, login, logout, register }: HeaderProps) {
  const [searchValue, setSearchValue] = useState('');
  const [airports, setAirports] = useState<{ key: string; value: string; label: string }[]>([]);
  const [modalType, toggle] = useToggle([undefined, 'login', 'register', 'reset']);
  const [_, setCoordinates] = useRecoilState(coordinatesState);
  const pathname = usePathname();
  const router = useRouter();

  async function onChange(value: string) {
    setSearchValue(value);
    const airportData = await getAirports({ icaos: [value], name: value });
    setAirports(
      airportData.data.map((airport) => ({
        key: airport.icao,
        value: airport.icao,
        label: `${airport.icao} - ${airport.name}`
      }))
    );
  }

  async function onClick(value: string) {
    setSearchValue('');
    // Get current path
    if (pathname == '/') {
      const airport = await getAirport({ icao: value });
      if (airport) {
        setCoordinates({ lat: airport.data.latitude, lon: airport.data.longitude });
      }
    } else {
      router.push(`/airport/${value}`)
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
          user={user}
          profilePicture={profilePicture}
          setProfilePicture={setProfilePicture}
          toggle={toggle}
          logout={logout}
        />
      </nav>
      <HeaderModal
        type={modalType}
        toggle={toggle}
        login={login}
        register={register}
      />
    </>
  );
}

interface UserSectionProps {
  user: User | undefined;
  profilePicture: File | undefined;
  setProfilePicture: SetterOrUpdater<File | undefined>;
  toggle: (type: string) => void;
  logout: () => Promise<void>;
}

function UserSection({ user, profilePicture, setProfilePicture, logout, toggle }: UserSectionProps) {

  return (
    <div className='user-section'>
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
                      onClick={logout}
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
    </div>
  )
}
