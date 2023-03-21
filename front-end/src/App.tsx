import { useEffect, useState } from "react";
import Device from "./Device";
import { DeviceStatuses, DeviceStatus } from "./types";

function App() {
  function wsStatus() {
    ws!.send("status ac:light");
  }

  function wsSwitch(turnOn: boolean, dev: string) {
    ws!.send(`switch ${dev} ${turnOn ? "on" : "off"}`);
  }

  function wsStartTimer(
    turnOn: boolean,
    dev: string,
    secondsToTrigger: number
  ) {
    if (secondsToTrigger === 0) {
      wsSwitch(turnOn, dev);
    }
    ws!.send(`timer:start ${dev} ${turnOn ? "on" : "off"} ${secondsToTrigger}`);
  }

  const [acData, setAcData] = useState<DeviceStatus>({} as DeviceStatus);
  function getUrl(): URL {
    if (import.meta.env.DEV) {
      return new URL("ws://localhost:8080/ws");
    }

    const url = new URL("/ws", window.location.href);
    url.protocol = url.protocol.replace("http", "ws");
    console.log(url);
    return url;
  }

  const [lightData, setLightData] = useState<DeviceStatus>({} as DeviceStatus);
  const [loading, setLoading] = useState<boolean>(true);
  const [ws, setWs] = useState<WebSocket | null>(null);
  const [selectedDevice, setSelectedDevice] = useState<"ac" | "light">("ac");
  const [secondsToTrigger, setSecondsToTrigger] = useState<number>(0);

  useEffect(() => {
    const ws = new WebSocket(getUrl());

    ws.onopen = () => {
      setLoading(false);
    };

    ws.onmessage = (ev) => {
      try {
        let data = JSON.parse(ev.data) as DeviceStatuses;
        const acData = data.data.filter((v) => v.name == "Ac");
        const lightData = data.data.filter((v) => v.name == "Light");
        if (acData.length === 1) {
          setAcData(acData && acData[0]);
        }
        if (lightData.length === 1) {
          setLightData(
            lightData && data.data.filter((v) => v.name == "Light")[0]
          );
        }
      } catch (e) {
        alert(
          `Error occured when got websocket message: ${e} - message: ${ev.data}`
        );
      }
    };

    setWs(ws);

    return () => {
      ws.close();
    };
  }, []);

  if (loading) {
    return <h1>Loading...</h1>;
  }

  const acTab = <Device data={acData} wsTurnOnDevice={wsSwitch} />;
  const lightTab = <Device data={lightData} wsTurnOnDevice={wsSwitch} />;

  return (
    <>
      <button onClick={wsStatus}>Tekan untuk update status</button>

      <div className="flex gap-12 items-center justify-center my-8 w-full">
        <button onClick={() => wsSwitch(false, "ac:light")}>
          Tekan untuk <span className="uppercase font-bold">mematikan</span>{" "}
          semua perangkat
        </button>
        <button onClick={() => wsSwitch(true, "ac:light")}>
          Tekan untuk <span className="uppercase font-bold">menyalakan</span>{" "}
          semua perangkat
        </button>
      </div>

      <div className="flex gap-12 items-center justify-center my-8 w-full">
        <button
          className={selectedDevice == "ac" ? "bg-orange-400 text-black" : ""}
          onClick={() => setSelectedDevice("ac")}
        >
          AC
        </button>
        <button
          className={
            selectedDevice == "light" ? "bg-orange-400 text-black" : ""
          }
          onClick={() => setSelectedDevice("light")}
        >
          Light
        </button>
      </div>

      {selectedDevice == "ac" ? acTab : lightTab}

      <form
        className="mt-8 flex flex-col items-center justify-center w-full"
        onSubmit={(e) => {
          e.preventDefault();
          wsStartTimer(true, "ac", secondsToTrigger);
        }}
      >
        <h1 className="font-bold mb-4 text-2xl">Timer</h1>
        <input className="text-black" onChange={(e) => {
          e.preventDefault();
          const { value } = e.target;
          const numValue = +value;
          if (Number.isNaN(numValue)) {
            return;
          }
          setSecondsToTrigger(numValue);
        }} required value={secondsToTrigger} />
        <button className="mt-2 bg-background-alt" type="submit">
          Pasang timer untuk {selectedDevice == 'ac' ? 'AC' : 'Lampu'}
        </button>
      </form>
    </>
  );
}

export default App;
