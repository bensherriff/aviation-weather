'use client';

import { Airport } from '@/app/_api/airport.types';
import { Metar } from '@/app/_api/metar.types';
import { FaArrowsSpin, FaLocationArrow } from 'react-icons/fa6';
import { Modal } from 'antd';
import Link from 'next/link';
import { AiFillStar, AiOutlineStar } from 'react-icons/ai';
import { useState } from 'react';

interface MetarDialogProps {
  airport: Airport;
  isOpen: boolean;
  onClose(): void;
}

export default function MetarDialog({ airport, isOpen, onClose }: MetarDialogProps) {
  const [isFavorite, setIsFavorite] = useState(false);

  function handleFavorite(value: boolean) {
    setIsFavorite(value);
  }

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
    <Modal
      title={
        <span className='flex justify-between'>
          <Link href={`/airport/${airport.icao}`}>
            {airport.icao} {airport.full_name}
          </Link>
          {isFavorite ? (
            <AiFillStar
              size={24}
              className='cursor-pointer text-blue-500 hover:text-blue-400'
              onClick={() => handleFavorite(false)}
            />
          ) : (
            <AiOutlineStar
              size={24}
              className='cursor-pointer text-blue-500 hover:text-blue-400'
              onClick={() => handleFavorite(true)}
            />
          )}
        </span>
      }
      open={isOpen}
      onCancel={onClose}
      closable={false}
      footer={[]}
      className='select-none'
    >
      <div className='min-w-0 flex-1'>
        <hr />
        <p className='text-sm font-medium text-gray-500'>{airport.metar?.raw_text}</p>
        <div className='mt-2 flex'>
          <span
            className={`flex inline-block align-middle text-sm text-white py-2 px-4 rounded-full 
              ${metarBGColor(airport.metar)}
            `}
          >
            {airport.metar?.flight_category ? airport.metar?.flight_category : 'UNKN'}
          </span>
          <div className='flex inline-block px-2'>
            <span className={`text-sm text-black ${windColor(airport.metar)} py-2 px-3 rounded-full`}>
              {airport.metar && airport.metar.wind_dir_degrees && Number(airport.metar.wind_dir_degrees) > 0 ? (
                <FaLocationArrow
                  className='align-middle'
                  style={{ rotate: `${-45 + 180 + Number(airport.metar.wind_dir_degrees)}deg` }}
                />
              ) : (
                <></>
              )}
              {airport.metar && airport.metar.wind_dir_degrees && airport.metar.wind_dir_degrees == 'VRB' ? (
                <FaArrowsSpin className='align-middle' />
              ) : (
                <></>
              )}
              <span className='align-middle pl-1.5'>
                {airport.metar?.wind_speed_kt != undefined && airport.metar?.wind_speed_kt > 0
                  ? `${airport.metar?.wind_speed_kt} KT`
                  : 'CALM'}
              </span>
            </span>
          </div>
        </div>
      </div>
    </Modal>
  );
}
