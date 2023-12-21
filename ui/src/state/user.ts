import { atom } from 'recoil';

export const favoritesState = atom({
  key: 'favoritesState',
  default: [] as string[]
});

export const profilePictureState = atom({
  key: 'profilePictureState',
  default: undefined as File | undefined
});
