use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;

#[derive(Clone, Copy, Serialize)]
pub enum Device {
    Ac,
    Light,
}

#[derive(Clone, Copy, Serialize)]
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
                timer_trigger_timestamp: 0,
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

    pub fn set_timer(&mut self, timer_timestamp_active: u64, is_turn_on: bool) {
        let mut timer = &mut self.timer;
        timer.is_set = true;
        timer.timer_trigger_timestamp = timer_timestamp_active;
        timer.timer_for_turn_on = is_turn_on;
    }

    pub fn stop_timer(&mut self) {
        let mut timer = &mut self.timer;
        timer.is_set = false;
        timer.timer_trigger_timestamp = 0;
        timer.timer_for_turn_on = false;
    }
}

#[derive(Clone, Copy, Serialize)]
pub struct DeviceTimer {
    is_set: bool,
    /// the timestamp for when the timer will trigger
    timer_trigger_timestamp: u64,
    /// true if the action will turn on the device
    timer_for_turn_on: bool,
}

#[allow(dead_code)]
impl DeviceTimer {
    /// the timestamp for when the timer will trigger
    pub fn timer_trigger_timestamp(&self) -> u64 {
        self.timer_trigger_timestamp
    }

    /// true if the action will turn on the device
    pub fn timer_for_turn_on(&self) -> bool {
        self.timer_for_turn_on
    }
}
