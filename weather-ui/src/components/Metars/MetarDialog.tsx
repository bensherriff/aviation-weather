import { Airport } from '@/js/api/airport.types';
import { Metar } from '@/js/api/metar.types';
import { FaArrowsSpin, FaLocationArrow } from 'react-icons/fa6';
import { Modal } from 'antd';

interface MetarDialogProps {
  airport: Airport;
  isOpen: boolean;
  onClose(): void;
}

export default function MetarDialog({ airport, isOpen, onClose }: MetarDialogProps) {
  function metarBGColor(metar: Metar | undefined) {
    if (metar?.flight_category == 'VFR') {
      return 'bg-emerald-600';
    } else if (metar?.flight_category == 'MVFR') {
      return 'bg-blue-600';
    } else if (metar?.flight_category == 'IFR') {
      return 'bg-red-600';
    } else if (metar?.flight_category == 'LIFR') {
      return 'bg-purple-600';
    } else {
      return 'bg-black';
    }
  }

  function windColor(metar: Metar | undefined) {
    if (Number(metar?.wind_speed_kt) <= 9) {
      return 'bg-green-300';
    } else if (Number(metar?.wind_speed_kt) > 9) {
      return 'bg-orange-300';
    } else if (Number(metar?.wind_speed_kt) > 12) {
      return 'bg-red-300';
    }
  }
  return (
    <Modal title={`${airport.icao} ${airport.full_name}`} open={isOpen} onCancel={onClose} footer={[]}>
      <div className='min-w-0 flex-1 select-none'>
        <hr />
        <p className='text-sm font-medium text-gray-500'>{airport.metar?.raw_text}</p>
        <div className='mt-2 flex'>
          <span
            className={`flex inline-block text-sm text-white ${metarBGColor(airport.metar)} py-2 px-4 rounded-full`}
          >
            {airport.metar?.flight_category ? airport.metar?.flight_category : 'UNKN'}
          </span>
          <div className='flex inline-block px-2'>
            <span className={`text-sm text-black ${windColor(airport.metar)} py-2 px-2 rounded-full`}>
              {airport.metar && airport.metar.wind_dir_degrees && Number(airport.metar.wind_dir_degrees) > 0 ? (
                <FaLocationArrow
                  className='pr-1'
                  style={{ rotate: `${-45 + 180 + Number(airport.metar.wind_dir_degrees)}deg` }}
                />
              ) : (
                <></>
              )}
              {airport.metar && airport.metar.wind_dir_degrees && airport.metar.wind_dir_degrees == 'VRB' ? (
                <FaArrowsSpin className='pr-1' />
              ) : (
                <></>
              )}
              {airport.metar?.wind_speed_kt != undefined && airport.metar?.wind_speed_kt > 0
                ? `${airport.metar?.wind_speed_kt} KT`
                : 'CALM'}
            </span>
          </div>
        </div>
      </div>
    </Modal>
  );
}
