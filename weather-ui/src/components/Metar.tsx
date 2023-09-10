import { Airport } from '@/js/api/airport.types';
import { getAirports } from '@/js/api/airport';
import { Metar } from '@/js/api/metar.types';
import { getMetars } from '@/js/api/metar';
import dynamic from 'next/dynamic';

export default async function Metar() {
  const Map = dynamic(() => import('@/components/MetarMap'), {
    loading: () => (
      <div className='grid min-h-full place-items-center px-6 py-24 sm:py-32 lg:px-8'>
        <div className='text-center'>
          <p className='mt-4 text-3xl font-bold tracking-tight text-gray-300 sm:text-5xl'>Loading...</p>
        </div>
      </div>
    ),
    ssr: false
  });

  let airports: Airport[] = [];

  async function update() {
    airports = await getAirports({ limit: 10, page: 1 });
    const metars = await getMetars(airports);
    for (let i = 0; i < metars.length; i++) {
      airports[i].metar = metars[i];
    }
  }
  await update();

  return <Map airportString={JSON.stringify(airports)} />;
}
