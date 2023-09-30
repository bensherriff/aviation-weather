'use client';

import { Airport } from '@/api/airport.types';
import { Metar } from '@/api/metar.types';
import { FaArrowsSpin, FaLocationArrow } from 'react-icons/fa6';
import Link from 'next/link';
import { AiFillStar, AiOutlineStar } from 'react-icons/ai';
import {
  BsFillCloudRainFill,
  BsFillCloudRainHeavyFill,
  BsFillCloudSleetFill,
  BsFillCloudSnowFill,
  BsQuestionLg
} from 'react-icons/bs';
import { useState } from 'react';
import { Grid, Modal, Tooltip } from '@mantine/core';
import './metars.css';
import SkyConditions from './SkyConditions';

interface MetarModalProps {
  airport: Airport;
  isOpen: boolean;
  onClose(): void;
}

export default function MetarModal({ airport, isOpen, onClose }: MetarModalProps) {
  const [isFavorite, setIsFavorite] = useState(false);

  function handleFavorite(value: boolean) {
    setIsFavorite(value);
  }

  return (
    <Modal opened={isOpen} onClose={onClose} withCloseButton={false} size={'50%'} className='modal'>
      <span className='title'>
        <Link href={`/airport/${airport.icao}`}>
          {airport.icao} {airport.full_name}
        </Link>
        {isFavorite ? (
          <AiFillStar size={24} className='star' onClick={() => handleFavorite(false)} />
        ) : (
          <AiOutlineStar size={24} className='star' onClick={() => handleFavorite(true)} />
        )}
      </span>
      <div className='min-w-0 flex-1'>
        <hr />
        {airport.metar && <MetarInfo metar={airport.metar} />}
      </div>
    </Modal>
  );
}

function MetarInfo({ metar }: { metar: Metar }) {
  function metarBGColor(metar: Metar | undefined) {
    if (metar?.flight_category == 'VFR') {
      return 'green';
    } else if (metar?.flight_category == 'MVFR') {
      return 'blue';
    } else if (metar?.flight_category == 'IFR') {
      return 'red';
    } else if (metar?.flight_category == 'LIFR') {
      return 'purple';
    } else {
      return 'black';
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
      <p style={{ fontWeight: '200', fontSize: '0.8em', color: 'gray' }}>{metar.raw_text}</p>
      <Grid gutter={18}>
        <Grid.Col className='gutter-row' span={6} style={{ marginTop: '0.5em' }}>
          <span
            style={{
              color: 'white',
              backgroundColor: metarBGColor(metar),
              borderRadius: '25px',
              padding: '0.4em 0.8em 0.4em 0.8em'
            }}
          >
            {metar.flight_category ? metar.flight_category : 'UNKN'}
          </span>
          <span style={{ marginLeft: '0.5em' }}>
            {metar.wind_speed_kt != undefined && metar.wind_speed_kt > 0 ? `${metar.wind_speed_kt} KT` : 'CALM'}
          </span>
        </Grid.Col>
        <Grid.Col className='gutter-row' span={6}>
          <SkyConditions metar={metar} />
        </Grid.Col>
        <Grid.Col className='gutter-row' span={12}>
          {metar.wx_string && metar.wx_string.split(' ').map((wx) => <MetarIcon wx={wx} />)}
        </Grid.Col>
      </Grid>
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
    <Tooltip label={title}>
      <span className={`rounded-full`}>{icon}</span>
    </Tooltip>
  );
}
