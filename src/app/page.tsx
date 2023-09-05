import React from 'react';
import { setAirport } from "@/js/state";
import { Airport } from "@/js/airport";
import Metar from '@/components/Metar';

// setAirport('KJYO', new Airport('Leesburg Executive Airport', 'KJYO'))
setAirport('KHEF', new Airport('Manassas Regional Airpoirt', 'KHEF', 38.724, -77517))
// setAirport('KIAD', new Airport('Dulles International Airport', 'KIAD'))
// setAirport('KFDK', new Airport('Frederick Municipal Airport', 'KFDK'))
// setAirport('KMRB', new Airport('Eastern West Virginia Regional Airport', 'KMRB'))
// setAirport('KOKV', new Airport('Winchester Regional Airport', 'KOKV'))
// setAirport('KFRR', new Airport('Front Royal-Warren County Airport', 'KFRR'))
// setAirport('KLUA', new Airport('Luray Caverns Airport', 'KLUA'))
// setAirport('KSHD', new Airport('Shenandoah Valley Airport', 'KSHD'))
// setAirport('KCHO', new Airport('Charlottesville-Albemarle Airport', 'KCHO'))
// setAirport('KCJR', new Airport('Culpeper Regional Airport', 'KCJR'))
// setAirport('KHWY', new Airport('Warrenton-Fauquier Airport', 'KHWY'))
// setAirport('KRMN', new Airport('Stafford Regional Airport', 'KRMN'))
// setAirport('KEZF', new Airport('Shannon Airport', 'KEZF'))
// setAirport('KDCA', new Airport('Ronald Reagan Washington National Airport', 'KDCA'))

export default function Page() {
    return <>
        <div>
            <Metar/>
        </div>
    </>
}
