import { useEffect, useState } from "react";
import "./App.css";
import { Device, getDevices } from "./api";
import { Link } from "react-router-dom";

function App() {
  const [devices, setDevices] = useState<Device[]>([]);
  console.log(devices);

  useEffect(() => {
    getDevices().then((data) => setDevices(data));
  }, []);

  return (
    <>
      <div>
        <h1>Devices</h1>
        <ul>
          {devices.map((device) => (
            <li key={device.id}>
              <Link to={`/device/${device.id}`}>{device.hostname}</Link>
            </li>
          ))}
        </ul>
      </div>
    </>
  );
}

export default App;
