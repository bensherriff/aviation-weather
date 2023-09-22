import axios, { AxiosResponse } from 'axios';

const serviceHost = process.env.SERVICE_HOST || 'http://localhost';
const servicePort = process.env.SERVICE_PORT || 5000;

export async function getRequest(endpoint: string, params: any): Promise<AxiosResponse<any, any> | undefined> {
  const response = await axios
    .get(`${serviceHost}:${servicePort}/${endpoint}`, { params })
    .catch((error) => console.error(error));
  return response || undefined;
}

export async function postRequest(endpoint: string, body: any): Promise<AxiosResponse<any, any> | undefined> {
  const response = await axios
    .post(`${serviceHost}:${servicePort}/${endpoint}`, { body })
    .catch((error) => console.error(error));
  return response || undefined;
}
