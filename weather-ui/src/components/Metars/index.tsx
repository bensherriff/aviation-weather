import { Metar } from '@/js/api/metar.types';
import dynamic from 'next/dynamic';

export default async function Metar({ className = '' }: { className?: string }) {
  const Map = dynamic(() => import('@/components/Metars/MetarMap'), {
    loading: () => (
      <div className='grid min-h-full place-items-center px-6 py-24 sm:py-32 lg:px-8'>
        <div className='text-center'>
          <p className='mt-4 text-3xl font-bold tracking-tight text-gray-300 sm:text-5xl'>Loading...</p>
        </div>
      </div>
    ),
    ssr: false
  });
  return <Map className={className} />;
}
