import { Metar } from '@/api/metar.types';
import { Skeleton } from '@mantine/core';
import dynamic from 'next/dynamic';

export default async function Metar() {
  const Map = dynamic(() => import('@/components/Metars/MetarMap'), {
    loading: () => (
      <Skeleton className='map-container' />
    ),
    ssr: false
  });
  return <Map />;
}
