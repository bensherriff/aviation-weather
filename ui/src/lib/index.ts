const baseUrl = import.meta.env.VITE_API_URL || 'http://localhost:8080/api';

export async function getRequest(endpoint: string, params: Record<string, any> = {}): Promise<Response> {
  Object.keys(params).forEach((key) => params[key] === undefined && delete params[key]);
  const urlParams = new URLSearchParams(params);
  const url = urlParams && urlParams.size > 0 ? `${baseUrl}/${endpoint}?${urlParams}` : `${baseUrl}/${endpoint}`;
  return await fetch(url, {
    method: 'GET',
    credentials: 'include'
  });
}

interface PostOptions {
  headers?: Record<string, any>;
  type?: 'json' | 'form';
}

export async function postRequest(endpoint: string, body?: any, options?: PostOptions): Promise<Response> {
  const url = `${baseUrl}/${endpoint}`;
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
  const url = `${baseUrl}/${endpoint}`;
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
  const url = `${baseUrl}/${endpoint}`;
  const response = await fetch(url, {
    method: 'DELETE',
    credentials: 'include'
  });
  return response;
}
