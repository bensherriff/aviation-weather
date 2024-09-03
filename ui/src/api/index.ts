const serviceHost = process.env.SERVICE_HOST || 'http://localhost';
const servicePort = process.env.SERVICE_PORT || 5000;
const baseURL = `${serviceHost}:${servicePort}`;

export async function getRequest(endpoint: string, params: Record<string, any> = {}): Promise<Response> {
  // Remove undefined params
  Object.keys(params).forEach((key) => params[key] === undefined && delete params[key]);
  const urlParams = new URLSearchParams(params);
  const url = urlParams && urlParams.size > 0 ? `${baseURL}/${endpoint}?${urlParams}` : `${baseURL}/${endpoint}`;
  const response = await fetch(url, {
    method: 'GET',
    credentials: 'include'
  });
  return response;
}

interface PostOptions {
  headers?: Record<string, any>;
  type?: 'json' | 'form';
}

export async function postRequest(endpoint: string, body?: any, options?: PostOptions): Promise<Response> {
  const url = `${baseURL}/${endpoint}`;
  let response;
  if (body && (!options?.type || options.type === 'json')) {
    response = await fetch(url, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      credentials: 'include',
      body: JSON.stringify(body)
    });
  } else {
    response = await fetch(url, {
      method: 'POST',
      credentials: 'include',
      body
    });
  }
  return response;
}

export async function putRequest(endpoint: string, body?: any, options?: PostOptions): Promise<Response> {
  const url = `${baseURL}/${endpoint}`;
  let response;
  if (body && (!options?.type || options.type === 'json')) {
    response = await fetch(url, {
      method: 'PUT',
      headers: {
        'Content-Type': 'application/json'
      },
      credentials: 'include',
      body: JSON.stringify(body)
    });
  } else {
    response = await fetch(url, {
      method: 'PUT',
      credentials: 'include',
      body
    });
  }
  return response;
}

export async function deleteRequest(endpoint: string): Promise<Response> {
  const url = `${baseURL}/${endpoint}`;
  const response = await fetch(url, {
    method: 'DELETE',
    credentials: 'include'
  });
  return response;
}

export interface Metadata {
  limit: number;
  page: number;
  total: number;
}
