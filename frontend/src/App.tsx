import { useEffect, useState } from "react";
import "./App.css";

const get = (endpoint: string) => {
  const uri = `http://localhost:8000/${endpoint}`;
  return fetch(uri).then((resp) => resp.json());
};

function App() {
  const [devices, setDevices] = useState([]);
  console.log(devices);

  useEffect(() => {
    get("device").then((data) => setDevices(data));
  }, []);

  return (
    <>
      <div>
        <h1>Devices</h1>
        <ul>
          {devices.map((device: any) => (
            <li key={device.id}>{device.hostname}</li>
          ))}
        </ul>
      </div>
    </>
  );
}

export default App;
