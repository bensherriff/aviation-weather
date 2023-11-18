import { deleteRequest, getRequest, postRequest } from '.';

export async function getPicture(): Promise<Blob | undefined> {
  const response = await getRequest('users/picture');
  if (response?.status === 200) {
    return response.blob();
  } else {
    return undefined;
  }
}

export async function setPicture(payload: File): Promise<boolean> {
  const data = new FormData();
  data.append('data', payload);
  // TODO: Figure out why the form data object is empty
  const response = await postRequest('users/picture', data, {
    type: 'form'
  });
  if (response?.status === 200) {
    return true;
  } else {
    return false;
  }
}

export async function getFavorites(): Promise<string[]> {
  const response = await getRequest('users/favorites');
  if (response?.status === 200) {
    return response.json();
  } else {
    return [];
  }
}

export async function addFavorite(icao: string): Promise<boolean> {
  const response = await postRequest(`users/favorites/${icao}`);
  if (response?.status === 200) {
    return true;
  } else {
    return false;
  }
}

export async function removeFavorite(icao: string): Promise<boolean> {
  const response = await deleteRequest(`users/favorites/${icao}`);
  if (response?.status === 200) {
    return true;
  } else {
    return false;
  }
}
