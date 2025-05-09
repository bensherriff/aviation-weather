import Cookies from 'js-cookie';
import { getRequest, postRequest } from '.';
import { RegisterUser, ResponseAuth, User } from './account.types';

export async function login(email: string, password: string): Promise<User | undefined> {
  const response = await postRequest('account/login', { email, password });
  if (response?.status === 200) {
    return response.json();
  } else {
    return undefined;
  }
}

export async function register(user: RegisterUser): Promise<boolean> {
  const response = await postRequest('account/register', user);
  if (response?.status === 201) {
    return true;
  } else {
    return false;
  }
}

export async function logout() {
  return await postRequest('account/logout', {});
}

export async function refresh(refresh_token_rotation?: boolean): Promise<ResponseAuth | undefined> {
  const response = await getRequest('account/refresh', { refresh_token_rotation });
  if (response?.status === 200) {
    return response.json();
  } else {
    return undefined;
  }
}

export async function me(): Promise<ResponseAuth | undefined> {
  const response = await getRequest('account/me');
  if (response?.status === 200) {
    return response.json();
  } else {
    return undefined;
  }
}

/**
 * Refreshes the logged_in cookie every interval. By default, the interval is 14 minutes.
 * @param interval
 * @returns interval id
 */
export function refreshLoggedIn(interval = 840000) {
  let loggedIn = Cookies.get('logged_in');
  const id = setInterval(async () => {
    const cookie = Cookies.get('logged_in');
    if (cookie != loggedIn) {
      loggedIn = cookie;
      const response = await refresh(true);
      if (!response) {
        Cookies.remove('logged_in');
      }
    }
  }, interval);
  return id;
}
