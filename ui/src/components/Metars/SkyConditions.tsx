'use client';

import { Metar, SkyCondition } from '@/api/metar.types';
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Filler,
  Legend
} from 'chart.js';
import { Line } from 'react-chartjs-2';

ChartJS.register(CategoryScale, LinearScale, PointElement, LineElement, Title, Tooltip, Filler, Legend);

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
  function createDataset(skyCondition: SkyCondition) {
    return {
      fill: true,
      label: skyCondition.sky_cover,
      data: [skyCondition.cloud_base_ft_agl, skyCondition.cloud_base_ft_agl],
      backgroundColor: skyConditionColor(skyCondition)
    };
  }
  if (metar.sky_condition && metar.sky_condition.length > 0) {
    console.log(metar);
    const options = {
      responsive: true,
      plugins: {
        legend: {
          display: false
        },
        title: {
          display: true,
          text: 'Sky Conditions'
        }
      }
    };
    const data = {
      labels: ['', ''],
      datasets: metar.sky_condition.map((skyCondition) => createDataset(skyCondition))
    };

    return <Line options={options} data={data} />;
  } else {
    return <></>;
  }
}
