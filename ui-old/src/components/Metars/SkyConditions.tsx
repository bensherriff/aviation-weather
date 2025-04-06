'use client';

import { Metar } from '@/api/metar.types';
import { Box, Card, Divider } from '@mantine/core';
import { CartesianGrid, LabelList, Line, LineChart, XAxis, YAxis } from 'recharts';

export default function SkyConditions({ metar }: { metar: Metar }) {
  const data: any = [
    {
      name: 'start'
    },
    {
      name: 'end'
    }
  ];
  if (metar.sky_condition && metar.sky_condition.length > 0 && metar.sky_condition[0].sky_cover != 'CLR') {
    let maxHeight = 0;
    metar.sky_condition.forEach((skyCondition, index) => {
      data[0][index] = skyCondition.cloud_base_ft_agl;
      data[1][index] = skyCondition.cloud_base_ft_agl;
      if (skyCondition.cloud_base_ft_agl > maxHeight) {
        maxHeight = skyCondition.cloud_base_ft_agl;
      }
    });
    maxHeight = Math.ceil((maxHeight % 1000 == 0 ? maxHeight + 1 : maxHeight) / 1000) * 1000;
    let interval;
    if (maxHeight <= 5000) {
      interval = 1;
    } else if (maxHeight <= 10000) {
      interval = 2;
    } else if (maxHeight <= 15000) {
      interval = 3;
    } else if (maxHeight <= 20000) {
      interval = 5;
    } else {
      interval = 10;
    }

    return (
      <Card padding='lg' radius='md'>
        <Divider my='sm' label='Sky Conditions' labelPosition='center' />
        <LineChart data={data} width={350} height={300} margin={{ top: 12, right: 8, left: 0, bottom: 0 }}>
          <CartesianGrid strokeDasharray='3 3' />
          <YAxis
            includeHidden
            ticks={[0, 1000 * interval, 2000 * interval, 3000 * interval, 4000 * interval, 5000 * interval]}
            domain={[0, maxHeight]}
          />
          <XAxis tick={false} />
          {metar.sky_condition.map((skyCondition, index) => (
            <Line
              key={`sky-condition-line-${index}`}
              type={'linear'}
              dataKey={index}
              dot={false}
              isAnimationActive={false}
            >
              <LabelList
                dataKey={index}
                position='insideRight'
                content={(props) => renderCustomizedLabel(props, skyCondition.sky_cover)}
              />
            </Line>
          ))}
        </LineChart>
      </Card>
    );
  } else {
    return (
      <Card>
        <Divider my='sm' label='Sky Conditions' labelPosition='center' />
        <Box style={{ width: '350px', height: '300px', textAlign: 'center' }}>Clear Skies</Box>
      </Card>
    );
  }
}

const renderCustomizedLabel = (props: any, skyCover: string) => {
  const { x, y, value, index } = props;
  if (index == 1) {
    return (
      <text x={x} y={y - 5} fill={'#285A64'} textAnchor='end'>
        {skyCover} {value}
      </text>
    );
  } else {
    return <></>;
  }
};
