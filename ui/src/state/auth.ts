import { User } from '@/api/auth.types';
import { atom } from 'recoil';

export const userState = atom({
  key: 'userState',
  default: undefined as User | undefined
});

export const isAuthenticatedState = atom({
  key: 'isAuthenticatedState',
  default: false
});
