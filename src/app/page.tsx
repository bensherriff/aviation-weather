import React from 'react';
import { airports } from "@/js/state";
import { Airport } from "@/js/airport";
import MetarCard from '@/components/MetarCard';

airports.set('KJYO', new Airport('Leesburg Executive Airport', 'KJYO'))
airports.set('KHEF', new Airport('Manassas Regional Airpoirt', 'KHEF'))
airports.set('KIAD', new Airport('Dulles International Airport', 'KIAD'))
airports.set('KFDK', new Airport('Frederick Municipal Airport', 'KFDK'))
airports.set('KMRB', new Airport('Eastern West Virginia Regional Airport', 'KMRB'))
airports.set('KOKV', new Airport('Winchester Regional Airport', 'KOKV'))
airports.set('KFRR', new Airport('Front Royal-Warren County Airport', 'KFRR'))
airports.set('KLUA', new Airport('Luray Caverns Airport', 'KLUA'))
airports.set('KSHD', new Airport('Shenandoah Valley Airport', 'KSHD'))
airports.set('KCHO', new Airport('Charlottesville-Albemarle Airport', 'KCHO'))
airports.set('KCJR', new Airport('Culpeper Regional Airport', 'KCJR'))
airports.set('KHWY', new Airport('Warrenton-Fauquier Airport', 'KHWY'))
airports.set('KRMN', new Airport('Stafford Regional Airport', 'KRMN'))
airports.set('KEZF', new Airport('Shannon Airport', 'KEZF'))

export default function Page() {
    return <>
        <div className="border-b border-gray-200 bg-gray-400 px-4 py-5 sm:px-6">
            <h3 className="text-base font-semibold leading-6 text-gray-900">Airports</h3>
        </div>
        <div className='p-4'>
            <MetarCard airports={[...airports.values()]}/>
        </div>
    </>
}
