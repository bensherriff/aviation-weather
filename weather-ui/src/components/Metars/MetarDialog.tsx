'use client';

import { Airport } from '@/api/airport.types';
import { Metar } from '@/api/metar.types';
import { FaArrowsSpin, FaLocationArrow } from 'react-icons/fa6';
import { Col, Grid, Modal, Row, Tooltip } from 'antd';
import Link from 'next/link';
import { AiFillStar, AiOutlineStar } from 'react-icons/ai';
import { BsFillCloudRainFill, BsFillCloudRainHeavyFill, BsFillCloudSleetFill, BsFillCloudSnowFill, BsQuestionLg } from 'react-icons/bs';
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
        {airport.metar && <MetarInfo metar={airport.metar} />}
        {/* <p className='text-sm font-medium text-gray-500'>{airport.metar?.raw_text}</p>
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
                <FaArrowsSpin className='align-middle pr-1.5' />
              ) : (
                <></>
              )}
              <span className='align-middle'>
                {airport.metar?.wind_speed_kt != undefined && airport.metar?.wind_speed_kt > 0
                  ? `${airport.metar?.wind_speed_kt} KT`
                  : `CALM`}
              </span>
            </span>
            {airport.metar?.wx_string?.split(' ').map((wx) => <MetarIcon wx={wx} />)}
          </div>
        </div> */}
      </div>
    </Modal>
  );
}

function MetarInfo({ metar }: { metar: Metar }) {
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
    if (metar) {
      if (Number(metar.wind_speed_kt) <= 9) {
        return 'bg-green-300';
      } else if (Number(metar.wind_speed_kt) <= 12) {
        return 'bg-orange-300';
      } else {
        return 'bg-red-300';
      }
    } else {
      return 'gb-gray-100';
    }
  }

  function metarIcon(weatherPhenomena: string) {
    if (weatherPhenomena == 'RA') {
      return <></>;
    } else {
      return <></>;
    }
  }

  return (
    <div>
      <p className='text-xs font-small text-gray-500'>{metar.raw_text}</p>
      <Row gutter={18}>
        <Col className='gutter-row' span={6}>
          <span
            className={`text-sm text-white py-2 px-4 rounded-full 
              ${metarBGColor(metar)}
            `}
          >
            {metar.flight_category ? metar.flight_category : 'UNKN'}
          </span>
        </Col>
        <Col className='gutter-row' span={12}>
          {metar.wx_string && metar.wx_string.split(' ').map((wx) => <MetarIcon wx={wx} />)}
        </Col>
      </Row>
      <Row gutter={2}>Compass TBD Compass TBD Compass TBD Compass TBD Compass TB</Row>
      <Row gutter={2}>
        <Col></Col>
      </Row>
    </div>
  );
}

function MetarIcon({ wx }: { wx: string }) {
  // let color = 'bg-gray-400';
  let title = '';
  let icon = undefined;
  if (wx.includes('DZ')) {
    title = 'Drizzle';
    icon = <BsFillCloudRainFill />;
  } else if (wx.includes('RA')) {
    title = 'Rain';
    icon = <BsFillCloudRainHeavyFill />;
  } else if (wx.includes('SN')) {
    title = 'Snow';
    icon = <BsFillCloudSnowFill />;
  } else if (wx.includes('SG')) {
    title = 'Snow Grains';
    icon = <BsFillCloudSnowFill />;
  } else if (wx.includes('IC')) {
    title = 'Ice Crystals';
    icon = <BsFillCloudSleetFill />;
  } else if (wx.includes('PL')) {
    title = 'Ice Pellets';
    icon = <BsFillCloudSleetFill />;
  } else if (wx.includes('GR')) {
    title = 'Hail';
    icon = <BsFillCloudSleetFill />;
  } else if (wx.includes('GS')) {
    title = 'Snow Pellets';
    icon = <BsFillCloudSleetFill />;
  } else if (wx.includes('BR')) {
    title = 'Mist';
    icon = <BsFillCloudRainHeavyFill />;
  } else if (wx.includes('FG')) {
    title = 'Fog';
    icon = <BsFillCloudRainHeavyFill />;
  } else if (wx.includes('FU')) {
    title = 'Smoke';
    icon = <BsFillCloudRainHeavyFill />;
  } else if (wx.includes('VA')) {
    title = 'Volcanic Ash';
    icon = <BsFillCloudRainHeavyFill />;
  } else if (wx.includes('DU')) {
    title = 'Dust';
    icon = <BsFillCloudRainHeavyFill />;
  } else if (wx.includes('SA')) {
    title = 'Sand';
    icon = <BsFillCloudRainHeavyFill />;
  } else if (wx.includes('HZ')) {
    title = 'Haze';
    icon = <BsFillCloudRainHeavyFill />;
  } else {
    title = 'Unknown';
    icon = <BsQuestionLg />;
  }

  // if (wx.charAt(0) == '+') {
  //   color = '';
  // } else if (wx.charAt(0) == '-') {
  //   color = '';
  // } else {
  //   color = '';
  // }
  return (
    <Tooltip title={title}>
      <span className={`rounded-full`}>{icon}</span>
    </Tooltip>
  );
}