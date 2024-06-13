import axios from 'axios';

const apiService = axios.create({
  baseURL: window.location.origin + "/api/v1",
});

export const get = (endpoint: string) => apiService.get(endpoint);

export const post = (endpoint: string, data: any) => apiService.post(endpoint, data);

export const put = (endpoint: string, data: any) => apiService.put(endpoint, data);

export const del = (endpoint: string) => apiService.delete(endpoint);

export default {
  get,
  post,
  put,
  del,
};
