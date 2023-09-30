'use client';

import { getAirport } from '@/api/airport';
import { Airport } from '@/api/airport.types';
import { getMetars } from '@/api/metar';
import { Metar } from '@/api/metar.types';
import SkyConditions from '@/components/Metars/SkyConditions';
import { useEffect, useState } from 'react';

export default function Page({ params }: { params: { icao: string } }) {
  const [airport, setAirport] = useState<Airport | undefined>(undefined);
  const [metar, setMetar] = useState<Metar | undefined>(undefined);

  useEffect(() => {
    async function loadAirport() {
      const { data: airportData } = await getAirport({ icao: params.icao });
      setAirport(airportData);
      const { data: metarData } = await getMetars([airportData.icao]);
      if (metarData.length > 0) {
        setMetar(metarData[0]);
      }
    }
    loadAirport();
  }, []);

  if (airport) {
    return (
      <>
        <div className=''>
          <h3 className=''>{airport.full_name}</h3>
          {metar && <SkyConditions metar={metar} />}
        </div>
      </>
    );
  } else {
    return <></>;
  }
}
