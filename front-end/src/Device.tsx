import { FC } from "react";
import { DeviceStatus } from "./types";

type Props = {
  data: DeviceStatus;
  wsTurnOnDevice: (turnOn: boolean, dev: string) => void;
};

const Device: FC<Props> = ({ data, wsTurnOnDevice }) => {
  const { name, status } = data;
  const device = name.toLowerCase();
  const date = new Date(status.lastTurnedOnTimestamp * 1000);
  const dateStr = `${date.getDate()}-${
    date.getMonth() + 1
  }-${date.getFullYear()} ${date.getHours()}.${date.getMinutes()}`;
  return (
    <>
      <ul>
        <li>Sedang menyala: {status.isOn ? "iya" : "tidak"}</li>
        {status.isOn && <li>Menyala sejak: {dateStr}</li>}
        <li>
          Timer dinyalakan untuk perangkat?{" "}
          {status.timer.isSet ? "iya" : "tidak"}
        </li>
      </ul>

      <button
        className="mt-8 w-full"
        onClick={() => wsTurnOnDevice(!status.isOn, device)}
      >
        {status.isOn ? "Matikan" : "Nyalakan"} {device == "ac" ? "AC" : "Lampu"}
      </button>
    </>
  );
};

export default Device;
