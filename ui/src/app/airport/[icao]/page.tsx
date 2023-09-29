'use client';

import { getAirport } from '@/api/airport';
import { Airport } from '@/api/airport.types';
import Link from 'next/link';
import { useEffect, useState } from 'react';

export default function Page({ params }: { params: { icao: string } }) {
  const [airport, setAirport] = useState<Airport | undefined>(undefined);

  useEffect(() => {
    async function loadAirport() {
      const { data: airport } = await getAirport({ icao: params.icao });
      setAirport(airport);
    }
    loadAirport();
  }, []);

  if (airport) {
    return (
      <>
        <div className=''>
          <h3 className=''>{airport.full_name}</h3>
          <Link href={'/'}>Back</Link>
        </div>
      </>
    );
  } else {
    return <></>;
  }
}
