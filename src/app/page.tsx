// 'use client';

import { Airport } from '@/js/airport';
import React from 'react';
import MetarCard from './components/MetarCard';

export default function Page() {
    // const [airports, setAirports] = useRecoilState(airportsState);

    // useEffect(() => {
    //     const defaultAirports = [
    //         new Airport('Leesburg Executive Airport', 'KJYO'),
    //         new Airport('Manassas Regional Airpoirt', 'KHEF'),
    //         new Airport('Dulles International Airport', 'KIAD'),
    //         new Airport('Frederick Municipal Airport', 'KFDK'),
    //         new Airport('Eastern West Virginia Regional Airport', 'KMRB'),
    //         new Airport('Winchester Regional Airport', 'KOKV'),
    //         new Airport('Front Royal-Warren County Airport', 'KFRR'),
    //         new Airport('Luray Caverns Airport', 'KLUA'),
    //         new Airport('Shenandoah Valley Airport', 'KSHD'),
    //         new Airport('Charlottesville-Albemarle Airport', 'KCHO'),
    //         new Airport('Culpeper Regional Airport', 'KCJR'),
    //         new Airport('Warrenton-Fauquier Airport', 'KHWY'),
    //         new Airport('Stafford Regional Airport', 'KRMN'),
    //         new Airport('Shannon Airport', 'KEZF'),
    //     ];
    //     setAirports(defaultAirports);
    // }, []);
    const defaultAirports = [
        new Airport('Leesburg Executive Airport', 'KJYO'),
        new Airport('Manassas Regional Airpoirt', 'KHEF'),
        new Airport('Dulles International Airport', 'KIAD'),
        new Airport('Frederick Municipal Airport', 'KFDK'),
        new Airport('Eastern West Virginia Regional Airport', 'KMRB'),
        new Airport('Winchester Regional Airport', 'KOKV'),
        new Airport('Front Royal-Warren County Airport', 'KFRR'),
        new Airport('Luray Caverns Airport', 'KLUA'),
        new Airport('Shenandoah Valley Airport', 'KSHD'),
        new Airport('Charlottesville-Albemarle Airport', 'KCHO'),
        new Airport('Culpeper Regional Airport', 'KCJR'),
        new Airport('Warrenton-Fauquier Airport', 'KHWY'),
        new Airport('Stafford Regional Airport', 'KRMN'),
        new Airport('Shannon Airport', 'KEZF'),
    ];
    

    return <>
        <div className="border-b border-gray-200 bg-gray-400 px-4 py-5 sm:px-6">
            <h3 className="text-base font-semibold leading-6 text-gray-900">Airports</h3>
        </div>
        <div className='p-4'>
            <MetarCard airports={defaultAirports}/>
        </div>
    </>;
}
