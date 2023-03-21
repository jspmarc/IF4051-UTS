export type DeviceStatus = {
  name: "Light" | "Ac";
  status: {
    isOn: boolean;
    lastTurnedOnTimestamp: number;
    timer: {
      isSet: boolean;
      secondsToTrigger: number;
      timerForTurnOn: boolean;
    };
  };
  error: string | null;
};

export type DeviceStatuses = {
  data: DeviceStatus[];
};
