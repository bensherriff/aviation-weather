import { getAirport } from '@/api/airport';
import Link from 'next/link';

export default async function Page({ params }: { params: { icao: string } }) {
  const { data: airport } = await getAirport({ icao: params.icao });
  return (
    <>
      <div className=''>
        <h3 className=''>{airport.full_name}</h3>
        <Link href={'/'}>Back</Link>
      </div>
    </>
  );
}
