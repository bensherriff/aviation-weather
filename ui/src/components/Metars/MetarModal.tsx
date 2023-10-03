'use client';

import { Airport } from '@/api/airport.types';
import { Metar } from '@/api/metar.types';
import { FaArrowsSpin, FaLocationArrow } from 'react-icons/fa6';
import Link from 'next/link';
import { AiFillStar, AiOutlineStar } from 'react-icons/ai';
import {
  BsFillCloudDrizzleFill,
  BsFillCloudFogFill,
  BsFillCloudHailFill,
  BsFillCloudHazeFill,
  BsFillCloudRainFill,
  BsFillCloudRainHeavyFill,
  BsFillCloudSleetFill,
  BsFillCloudSnowFill,
  BsQuestionLg
} from 'react-icons/bs';
import { useState } from 'react';
import { Card, Divider, Grid, Modal, Tooltip } from '@mantine/core';
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
        <Divider style={{ paddingTop: '0.1em' }} />
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
        return 'green';
      } else if (Number(metar.wind_speed_kt) <= 12) {
        return 'orange';
      } else {
        return 'red';
      }
    } else {
      return 'gray';
    }
  }

  return (
    <div>
      <p style={{ fontWeight: '200', fontSize: '0.8em', color: 'gray' }}>{metar.raw_text}</p>
      <Grid gutter={18}>
        <Grid.Col className='gutter-row' span={6} style={{ marginTop: '0.5em' }}>
          <Grid.Col span={12}>
            <Grid style={{ padding: '2px' }}>
              <Grid.Col span={6}>
                <Card
                  shadow='sm'
                  padding='sm'
                  radius='md'
                  style={{
                    backgroundColor: metarBGColor(metar),
                    textAlign: 'center',
                    color: 'white'
                  }}
                >
                  {metar.flight_category ? metar.flight_category : 'UNKN'}
                </Card>
              </Grid.Col>
              <Grid.Col span={6}>
                <>
                  {metar.wind_speed_kt == undefined || metar.wind_speed_kt == 0 ? (
                    <Card
                      shadow='sm'
                      padding='sm'
                      radius='md'
                      style={{ textAlign: 'center', backgroundColor: 'green', color: 'white' }}
                    >
                      CALM
                    </Card>
                  ) : (
                    <Card shadow='sm' padding='sm' radius='md' style={{ textAlign: 'center' }}>
                      <Card.Section
                        style={{
                          backgroundColor: windColor(metar),
                          color: 'white'
                        }}
                      >
                        <span style={{ display: 'inline-block' }}>{metar.wind_speed_kt} KT</span>
                      </Card.Section>
                      <Card.Section>
                        {metar.wind_dir_degrees && Number(metar.wind_dir_degrees) > 0 ? (
                          <>
                            <FaLocationArrow style={{ rotate: `${-45 + 180 + Number(metar.wind_dir_degrees)}deg` }} />
                            {metar.wind_dir_degrees}&#176;
                          </>
                        ) : (
                          <></>
                        )}
                        {metar.wind_dir_degrees && metar.wind_dir_degrees == 'VRB' ? (
                          <>
                            <FaArrowsSpin />
                            VRB
                          </>
                        ) : (
                          <></>
                        )}
                      </Card.Section>
                    </Card>
                  )}
                </>
              </Grid.Col>
            </Grid>
          </Grid.Col>
          <Grid.Col className='gutter-row' span={12}>
            <Grid style={{ paddingTop: '1em', paddingBottom: '1em' }} gutter={48}>
              {metar.wx_string &&
                metar.wx_string.split(' ').map((wx) => (
                  <Grid.Col span={1}>
                    <MetarIcon wx={wx} />
                  </Grid.Col>
                ))}
            </Grid>
          </Grid.Col>
        </Grid.Col>
        <Grid.Col className='gutter-row' span={6}>
          <SkyConditions metar={metar} />
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
    icon = <BsFillCloudHailFill />;
  } else if (wx.includes('GS')) {
    title = 'Snow Pellets';
    icon = <BsFillCloudSleetFill />;
  } else if (wx.includes('BR')) {
    title = 'Mist';
    icon = <BsFillCloudDrizzleFill />;
  } else if (wx.includes('FG')) {
    title = 'Fog';
    icon = <BsFillCloudFogFill />;
  } else if (wx.includes('FU')) {
    title = 'Smoke';
    icon = <BsFillCloudHazeFill />;
  } else if (wx.includes('VA')) {
    title = 'Volcanic Ash';
    icon = <BsFillCloudHazeFill />;
  } else if (wx.includes('DU')) {
    title = 'Dust';
    icon = <BsFillCloudHazeFill />;
  } else if (wx.includes('SA')) {
    title = 'Sand';
    icon = <BsFillCloudHazeFill />;
  } else if (wx.includes('HZ')) {
    title = 'Haze';
    icon = <BsFillCloudHazeFill />;
  } else {
    title = 'Unknown';
    icon = <BsQuestionLg />;
  }

  return (
    <Tooltip label={title}>
      <span
        style={{
          color: 'white',
          backgroundColor: 'CornflowerBlue',
          borderRadius: '25px',
          padding: '0.6em 0.7em 0.6em 0.7em'
        }}
      >
        {icon}
      </span>
    </Tooltip>
  );
}
