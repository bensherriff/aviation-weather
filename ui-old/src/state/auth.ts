import { User } from '@/api/auth.types';
import { atom, selector } from 'recoil';

export const userState = atom({
  key: 'userState',
  default: undefined as User | undefined
});

export const isAdminState = selector({
  key: 'isAdminState',
  get: ({ get }) => {
    const user = get(userState);
    return user?.role === 'admin';
  }
});

export const refreshIdState = atom({
  key: 'refreshIdState',
  default: undefined as NodeJS.Timeout | undefined
});
