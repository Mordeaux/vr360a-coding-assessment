const get = async (endpoint: string) => {
  const uri = `http://localhost:8000/${endpoint}`;
  return fetch(uri).then((resp) => resp.json());
};

export interface Device {
  id: number;
  hostname: string;
}

export interface DeviceInfo {
  id: number;
  total_memory: number;
  used_memory: number;
  total_swap: number;
  used_swap: number;
  system_name: string;
  kernel_version: string;
  os_version: string;
  hostname: string;
  number_of_cpus: number;
  timestamp: number;
  device_id: number;
}

export const getDevices = (): Promise<Device[]> => get("device");

export const getDeviceInfos = (deviceId: string): Promise<DeviceInfo[]> =>
  get(`device/${deviceId}/device_info`);
