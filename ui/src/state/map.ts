import { Coordinate } from '@/api/airport.types';
import { atom } from 'recoil';

export const coordinatesState = atom({
  key: 'coordinatesState',
  default: { lat: 38.7209, lon: -77.5133 } as Coordinate
});

export const zoomState = atom({
  key: 'zoomState',
  default: 8
});
