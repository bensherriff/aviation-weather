'use client';

import Link from 'next/link';
import { AiOutlineUser } from 'react-icons/ai';
import { useState } from 'react';
import { getAirports } from '@/api/airport';
import { useRouter } from 'next/navigation';
import { Autocomplete, Avatar } from '@mantine/core';
import './topbar.css';

export default function Topbar() {
  const [searchValue, setSearchValue] = useState('');
  const [airports, setAirports] = useState<{ key: string; value: string; label: string }[]>([]);
  const router = useRouter();

  async function onChange(value: string) {
    setSearchValue(value);
    const airportData = await getAirports({ filter: value });
    setAirports(
      airportData.data.map((airport) => ({
        key: airport.icao,
        value: airport.icao,
        label: `${airport.icao} - ${airport.full_name}`
      }))
    );
  }

  function onClick(value: string) {
    router.push(`/airport/${value}`);
    setSearchValue('');
  }

  return (
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
      <Link className='avatar' href={'/profile'}>
        <Avatar variant='filled'>
          <AiOutlineUser />
        </Avatar>
      </Link>
    </nav>
  );
}
