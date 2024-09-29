import { useLoaderData, ActionFunctionArgs } from "react-router-dom";
import { DeviceInfo, getDeviceInfos } from "./api";

interface LoaderData {
  deviceInfos: DeviceInfo[];
}

export async function loader({
  params,
}: ActionFunctionArgs): Promise<LoaderData> {
  if (!params.deviceId) throw new Error("Device ID is required");

  const deviceInfos = await getDeviceInfos(params.deviceId);
  return { deviceInfos };
}

const Device = () => {
  const { deviceInfos } = useLoaderData() as LoaderData;
  return (
    <div>
      <h1>Device Info: {deviceInfos[0].hostname}</h1>
      <ul>
        {deviceInfos.map((deviceInfo) => (
          <li key={deviceInfo.id}>
            <h3>
              Time: {new Date(deviceInfo.timestamp * 1000).toLocaleString()}
            </h3>
            <p>
              System: {deviceInfo.system_name} - Total Memory:{" "}
              {deviceInfo.total_memory} - Used Memory: {deviceInfo.used_memory}{" "}
              - Total Swap: {deviceInfo.total_swap} - Used Swap:{" "}
              {deviceInfo.used_swap} - System Name: {deviceInfo.system_name} -
              Kernel Version: {deviceInfo.kernel_version} - OS Version:{" "}
              {deviceInfo.os_version} - Number of CPUs:{" "}
              {deviceInfo.number_of_cpus}
            </p>
          </li>
        ))}
      </ul>
    </div>
  );
};

export default Device;
