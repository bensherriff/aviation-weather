'use client';

import { AutoComplete, Avatar } from 'antd';
import Link from 'next/link';
import { AiOutlineUser } from 'react-icons/ai';
import { useState } from 'react';
import { getAirports } from '@/app/_api/airport';
import { useRouter } from 'next/navigation';

const DEFAULT_ICON_SIZE = 40;

export default function Topbar() {
  const [searchValue, setSearchValue] = useState('');
  const [airports, setAirports] = useState<{ key: string; value: string; label: string }[]>([]);
  const router = useRouter();

  async function onSearch(value: string) {
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

  function onSelect(value: string) {
    setSearchValue('');
    router.push(`/airport/${value}`);
  }

  return (
    <>
      <nav className='w-screen flex bg-gray-700 text-gray-200 justify-between'>
        <div className='flex'>
          <Link href={'/'} className='align-middle pt-2.5 px-6 text-lg'>
            <span>Aviation Weather</span>
          </Link>
          <AutoComplete
            className='w-72 relative top-2'
            autoFocus
            defaultActiveFirstOption
            value={searchValue}
            options={airports}
            onSelect={onSelect}
            onSearch={onSearch}
            onBlur={() => setSearchValue('')}
            placeholder='Search Airports...'
          />
        </div>
        <Link className='my-1 mr-2' href={'/profile'}>
          <Avatar shape='circle' size={DEFAULT_ICON_SIZE} icon={<AiOutlineUser />} />
        </Link>
      </nav>
    </>
  );
}
