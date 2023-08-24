import { Airport } from "@/js/airport";
import { Metar, getMetars } from "@/js/weather"
import Link from "next/link"
  
export default async function MetarCard({airports}: {airports: Airport[]}) {
    // await getMetars(defaultAirports).then((result) => {
    //     setMetars(result);
    // });
    const metars = await getMetars(airports);
    for (let i = 0; i < airports.length; i++) {
        airports[i].metar = metars[i];
    }

    function metarBGColor(metar: Metar | undefined) {
        if (metar?.flight_category == 'VFR') {
            return 'bg-emerald-600'
        } else if (metar?.flight_category == 'MVFR') {
            return 'bg-blue-600'
        } else if (metar?.flight_category == 'IFR') {
            return 'bg-red-600'
        } else if (metar?.flight_category == 'LIFR') {
            return 'bg-purple-600'
        } else {
            return 'bg-black'
        }
    }

    return (
        <div className="grid grid-cols-1 gap-4 sm:grid-cols-2">
            {airports.map((airport) => (
                <div
                    key={airport.metar?.station_id}
                    className={`relative flex items-center space-x-3 rounded-lg border border-gray-300 bg-white px-4 py-2 shadow-sm focus-within:ring-2 focus-within:ring-indigo-500 focus-within:ring-offset-2 hover:border-gray-400`}
                >
                    <div className="min-w-0 flex-1">
                    <Link href={'#'}>
                        <span className="absolute inset-0" aria-hidden="true" />
                        <p className="text-gray-900 pb-1">{airport.metar?.station_id} - <span>{airport.name}</span></p>
                        <p className='text-sm font-medium text-gray-500'>{airport.metar?.raw_text}</p>
                        <div className='mt-2'>
                            <span className={`truncate text-sm text-white ${metarBGColor(airport.metar)} inline-block py-2 px-4 rounded-full`}>{airport.metar?.flight_category}</span>
                        </div>
                    </Link>
                    </div>
                </div>
            ))}
        </div>
    )
  }