import { Airport } from "@/js/airport";
import { getAirports, setAirport } from "@/js/state";
import { Metar, getMetars } from "@/js/weather"
import Link from "next/link"
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'
import { faArrowsSpin, faLocationArrow } from '@fortawesome/free-solid-svg-icons'

export default async function MetarGrid() {
    const airports: Airport[] = getAirports();

    async function update() {
        const airports: Airport[] = getAirports();
        const metars = await getMetars(airports);
        for (let i = 0; i < airports.length; i++) {
            airports[i].metar = metars[i];
            setAirport(airports[i].icao, airports[i]);
        }
        // setTimeout(update, 30 * 60 * 1000);
    //     setTimeout(update, 5000);
    }
    await update();
    

    return (
        <div className="grid grid-cols-1 gap-4 sm:grid-cols-2">
            {airports.map((airport) => (
                <MetarCard airport={airport}/>
            ))}
        </div>
    );
}

function MetarCard({ airport}: { airport: Airport}) {
    function metarBGColor(metar: Metar | undefined) {
        if (metar?.flight_category == 'VFR') {
            return 'bg-emerald-600'
        } else if (metar?.flight_category == 'MVFR') {
            return 'bg-blue-600'
        } else if (metar?.flight_category == 'IFR') {
            return 'bg-orange-600'
        } else if (metar?.flight_category == 'LIFR') {
            return 'bg-red-600'
        } else {
            return 'bg-black'
        }
    }

    function windColor(metar: Metar | undefined) {
        if (Number(metar?.wind_speed_kt) <= 12) {
            return 'bg-green-300';
        } else if (Number(metar?.wind_speed_kt) > 12) {
            return 'bg-orange-300';
        }
    }

    return (
        <div
            key={airport.metar?.station_id}
            className={`relative flex items-center space-x-3 rounded-lg border border-gray-300 bg-white px-4 py-2 shadow-sm focus-within:ring-2 focus-within:ring-indigo-500 focus-within:ring-offset-2 hover:border-gray-400`}
        >
            <div className="min-w-0 flex-1">
            <Link href={`/airport/${airport.icao}`}>
                <span className="absolute inset-0" aria-hidden="true" />
                <p className="text-gray-900 pb-1"><span className='font-semibold'>{airport.icao}</span> {airport.name}</p>
                <hr/>
                <p className='text-sm font-medium text-gray-500'>{airport.metar?.raw_text}</p>
                <div className='mt-2'>
                    <span className={`truncate text-sm text-white ${metarBGColor(airport.metar)} inline-block py-2 px-4 rounded-full`}>{airport.metar?.flight_category? airport.metar?.flight_category : 'UNKN'}</span>
                    <span className="truncate inline-block py-2 px-2">
                        <span className={`text-black ${windColor(airport.metar)} p-2 rounded-full mr-1`}>
                            {airport.metar && airport.metar.wind_dir_degrees && Number(airport.metar.wind_dir_degrees) > 0?
                                <FontAwesomeIcon className="pr-1" icon={faLocationArrow} style={{rotate: `${-45  + 180 + Number(airport.metar.wind_dir_degrees)}deg`}}/>: <></>
                            }
                            {airport.metar && airport.metar.wind_dir_degrees && airport.metar.wind_dir_degrees == 'VRB'?
                                <FontAwesomeIcon className="pr-1" icon={faArrowsSpin}/>: <></>
                            }
                            {airport.metar?.wind_speed_kt != undefined && airport.metar?.wind_speed_kt > 0? `${airport.metar?.wind_speed_kt} KT` : "CALM"}
                        </span>
                    </span>
                </div>
            </Link>
            </div>
        </div>
    );
}