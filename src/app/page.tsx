import React from 'react';
import { getAirports, setAirport } from "@/js/state";
import { Airport } from "@/js/airport";
import Metar from '@/components/Metar';

setAirport('KJYO', new Airport('Leesburg Executive Airport', 'KJYO'))
setAirport('KHEF', new Airport('Manassas Regional Airpoirt', 'KHEF'))
setAirport('KIAD', new Airport('Dulles International Airport', 'KIAD'))
setAirport('KFDK', new Airport('Frederick Municipal Airport', 'KFDK'))
setAirport('KMRB', new Airport('Eastern West Virginia Regional Airport', 'KMRB'))
setAirport('KOKV', new Airport('Winchester Regional Airport', 'KOKV'))
setAirport('KFRR', new Airport('Front Royal-Warren County Airport', 'KFRR'))
setAirport('KLUA', new Airport('Luray Caverns Airport', 'KLUA'))
setAirport('KSHD', new Airport('Shenandoah Valley Airport', 'KSHD'))
setAirport('KCHO', new Airport('Charlottesville-Albemarle Airport', 'KCHO'))
setAirport('KCJR', new Airport('Culpeper Regional Airport', 'KCJR'))
setAirport('KHWY', new Airport('Warrenton-Fauquier Airport', 'KHWY'))
setAirport('KRMN', new Airport('Stafford Regional Airport', 'KRMN'))
setAirport('KEZF', new Airport('Shannon Airport', 'KEZF'))
setAirport('KDCA', new Airport('Ronald Reagan Washington National Airport', 'KDCA'))
// setAirport('KMQI', new Airport('Test Airport', 'KMQI'))
// setAirport('KEKQ', new Airport('Test Airport', 'KEKQ'))
// setAirport('KCSV', new Airport('Test Airport', 'KCSV'))

export default function Page() {
    

    return <>
        <div className="bg-gray-700 px-4 py-1 sm:px-6">
            <h3 className="text-lg font-bold leading-6 text-gray-200">Metar Map</h3>
        </div>
        <div>
            <Metar/>
        </div>
    </>
}
