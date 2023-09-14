'use client';

import { Avatar } from 'antd';
import Search from 'antd/es/input/Search';
import { useRouter } from 'next/navigation';
import { AiOutlineUser } from 'react-icons/ai';

export default function Topbar() {
  const router = useRouter();

  function onSearch(value: string) {
    router.push(`/airports/${value}`);
  }

  return (
    <nav className='w-screen flex bg-gray-700 text-gray-200'>
      <Search
        placeholder='Search Airports...'
        onSearch={onSearch}
        enterButton
        className='p-2'
        style={{ width: '20em' }}
      />
      <Avatar shape='square' size={48} icon={<AiOutlineUser />} />
    </nav>
  );
}
