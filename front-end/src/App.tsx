import { useEffect, useState } from "react";
import "./App.css";

type DeviceStatus = {
  name?: "Light" | "Ac";
  status?: {
    isOn: boolean;
    lastTurnedOnTimestamp: number;
    timer: {
      isSet: boolean;
      secondsToTrigger: number;
      timerForTurnOn: boolean;
    };
  };
  error?: string | null;
}[];

function App() {
  function getStatusButton() {
    console.log("sending");
    ws!.send("status light:ac");
    console.log("sent");
  }

  function getUrl(): URL {
    if (import.meta.env.DEV) {
      return new URL("ws://localhost:8080/ws");
    }

    const url = new URL("/ws", "/localhost:8080");
    url.protocol.replace("http", "ws");
    return url;
  }

  const [data, setData] = useState<DeviceStatus>({});
  const [loading, setLoading] = useState<boolean>(true);
  const [ws, setWs] = useState<WebSocket | null>(null);

  useEffect(() => {
    const ws = new WebSocket(getUrl());

    ws.onopen = (ev) => {
      console.log("connected", ev);
      setLoading(false);
    };

    ws.onmessage = (ev) => {
      let data = JSON.parse(ev.data) as DeviceStatus;
      console.log(data);
      setData(data);
    };

    setWs(ws);

    return () => {
      ws.close();
    };
  }, []);

  if (loading) {
    return <h1>Loading...</h1>;
  }

  return (
    <>
      <button onClick={getStatusButton}>Press me to get new status</button>
      <pre>{JSON.stringify(data, null, 4)}</pre>
    </>
  );
}

export default App;
