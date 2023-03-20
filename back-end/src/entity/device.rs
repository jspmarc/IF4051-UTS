use super::Error;
use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Copy, Serialize)]
#[repr(u8)]
pub enum Device {
    Ac,
    Light,
}

impl Device {
    pub fn from_str(value: &str) -> Result<Self, Error> {
        match value.to_lowercase().trim() {
            "ac" => Ok(Device::Ac),
            "light" => Ok(Device::Light),
            dev => Err(Error::UnknownDevice(dev.to_string())),
        }
    }
}

#[macro_export]
macro_rules! device_from_str {
    ($str:expr) => {
        match Device::from_str($str) {
            Ok(dev) => dev,
            Err(err) => return Err(err),
        }
    };
}

impl ToString for Device {
    fn to_string(&self) -> String {
        match *self {
            Device::Ac => String::from("ac"),
            Device::Light => String::from("light"),
        }
    }
}

#[derive(Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceState {
    is_on: bool,
    last_turned_on_timestamp: u64,
    timer: DeviceTimer,
}

impl DeviceState {
    pub fn new() -> Self {
        Self {
            is_on: false,
            last_turned_on_timestamp: 0,
            timer: DeviceTimer {
                is_set: false,
                seconds_to_trigger: 0,
                timer_for_turn_on: false,
            },
        }
    }

    pub fn turn_on(&mut self) {
        self.is_on = true;
        let start = SystemTime::now();
        let timestamp = start.duration_since(UNIX_EPOCH).unwrap();
        self.last_turned_on_timestamp = timestamp.as_secs();
    }

    pub fn turn_off(&mut self) {
        self.is_on = false;
        self.last_turned_on_timestamp = 0;
    }

    pub fn is_timer_set(&self) -> bool {
        self.timer.is_set
    }

    pub fn set_timer(&mut self, timer_timestamp_active: u64, is_turn_on: bool) {
        let mut timer = &mut self.timer;
        timer.is_set = true;
        timer.seconds_to_trigger = timer_timestamp_active;
        timer.timer_for_turn_on = is_turn_on;
    }

    pub fn stop_timer(&mut self) {
        let mut timer = &mut self.timer;
        timer.is_set = false;
        timer.seconds_to_trigger = 0;
        timer.timer_for_turn_on = false;
    }
}

#[derive(Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceTimer {
    is_set: bool,
    seconds_to_trigger: u64,
    /// true if the action will turn on the device
    timer_for_turn_on: bool,
}

#[allow(dead_code)]
impl DeviceTimer {
    /// true if the action will turn on the device
    pub fn timer_for_turn_on(&self) -> bool {
        self.timer_for_turn_on
    }
}
