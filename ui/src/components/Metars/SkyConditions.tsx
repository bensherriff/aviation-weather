'use client';

import { Metar, SkyCondition } from '@/api/metar.types';
import { Box } from '@mantine/core';
import {
  Customized,
  Label,
  LabelList,
  Line,
  LineChart,
  ReferenceLine,
  ResponsiveContainer,
  XAxis,
  YAxis
} from 'recharts';

export default function SkyConditions({ metar }: { metar: Metar }) {
  function skyConditionColor(skyCondition: SkyCondition) {
    if (skyCondition.sky_cover == 'CLR') {
      return '#FFFFFF';
    } else if (skyCondition.sky_cover == 'FEW') {
      return '#19c4e6';
    } else if (skyCondition.sky_cover == 'SCT') {
      return '#6119e6';
    } else if (skyCondition.sky_cover == 'BKN') {
      return '#e6c019';
    } else if (skyCondition.sky_cover == 'OVC') {
      return '#e68019';
    } else {
      return '#e6194b';
    }
  }
  if (metar.sky_condition && metar.sky_condition.length > 0) {
    const data: any = [
      {
        name: 'start'
      },
      {
        name: 'end'
      }
    ];
    metar.sky_condition.forEach((skyCondition, index) => {
      data[0][index] = skyCondition.cloud_base_ft_agl;
      data[1][index] = skyCondition.cloud_base_ft_agl;
    });

    return (
      <LineChart data={data} width={350} height={300}>
        <YAxis domain={[0, (dataMax: number) => (dataMax < 1000 ? 1000 : Math.ceil(dataMax / 1000) * 1000)]} />
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
    );
  } else {
    return <></>;
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
