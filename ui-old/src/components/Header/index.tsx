'use client';

import Link from 'next/link';
import { useState } from 'react';
import { getAirport, getAirports } from '@/api/airport';
import { Autocomplete, Button, Group, UnstyledButton } from '@mantine/core';
import { SetterOrUpdater, useRecoilState } from 'recoil';
import { useToggle } from '@mantine/hooks';
import { HeaderModal } from './HeaderModal';
import { coordinatesState } from '@/state/map';
import { User } from '@/api/auth.types';
import { usePathname, useRouter } from 'next/navigation';
import { FaMoon } from "react-icons/fa6";
import { FaSun } from "react-icons/fa6";
import UserMenu from './UserMenu';
import './styles.css';

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
        <div className='right'>
          <UnstyledButton style={{ paddingRight: '1em', margin: 'auto' }}>
            <FaMoon />
            {/* <FaSun /> */}
          </UnstyledButton>
          {user ? (
            <UserMenu
              user={user}
              profilePicture={profilePicture}
              setProfilePicture={setProfilePicture}
              toggle={toggle}
              logout={logout}
            />
          ) : (
            <Group className='user'>
              <Button onClick={() => toggle('login')}>Login</Button>
              <Button variant='outline' onClick={() => toggle('register')}>
                Sign up
              </Button>
            </Group>
          )}
        </div>
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
