'use client';

import { AutoComplete, Avatar, Modal } from 'antd';
import Link from 'next/link';
import { AiOutlineSearch, AiOutlineUser } from 'react-icons/ai';
import { Button } from '@blueprintjs/core';
import { useState } from 'react';
import { getAirports } from '@/app/_api/airport';
import { useRouter } from 'next/navigation';

const DEFAULT_ICON_SIZE = 40;

export default function Topbar() {
  const [modalOpen, setModalOpen] = useState(false);
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
    setModalOpen(false);
    setSearchValue('');
    router.push(`/airport/${value}`);
  }

  function onClose() {
    setModalOpen(false);
    setSearchValue('');
  }

  return (
    <>
      <Modal
        open={modalOpen}
        closable={false}
        onCancel={onClose}
        footer={[]}
        className='p-0'
        title={'Search for Airports'}
      >
        <AutoComplete
          className='w-full'
          allowClear
          autoFocus
          value={searchValue}
          options={airports}
          onSelect={onSelect}
          onSearch={onSearch}
          placeholder='Search Airports...'
        />
      </Modal>
      <nav className='w-screen flex bg-gray-700 text-gray-200 justify-between'>
        <div className='flex'>
          <Link href={'/'} className='align-middle pt-2.5 pl-6 text-lg'>
            <span>Aviation Weather</span>
          </Link>
          <Button
            icon={<AiOutlineSearch size={24} className='float-left mr-2 hover:text-white' />}
            className='my-1 ml-6 pl-10 pr-12 border-none rounded-lg bg-gray-800 text-base text-gray-200/75 hover:bg-gray-600 hover:text-white cursor-pointer'
            onClick={() => setModalOpen(true)}
          >
            Search Airports...
          </Button>
        </div>
        <Link className='my-1 mr-2' href={'/profile'}>
          <Avatar shape='circle' size={DEFAULT_ICON_SIZE} icon={<AiOutlineUser />} />
        </Link>
      </nav>
    </>
  );
}
