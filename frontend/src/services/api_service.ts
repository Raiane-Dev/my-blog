import axios from 'axios';

const apiService = axios.create({
  baseURL: "/api/v1",
});

const get = (endpoint: string) => apiService.get(endpoint);

const post = (endpoint: string, data: any) => apiService.post(endpoint, data);

const put = (endpoint: string, data: any) => apiService.put(endpoint, data);

const del = (endpoint: string) => apiService.delete(endpoint);

export {
  get,
  post,
  put,
  del,
};
