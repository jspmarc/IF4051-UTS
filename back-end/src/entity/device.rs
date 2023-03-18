use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::Serialize;

#[derive(Clone, Copy, Serialize)]
pub enum Device {
    Ac,
    Light,
}

#[derive(Clone, Copy, Serialize)]
pub struct DeviceState {
    is_on: bool,
    is_timer_set: bool,
    last_turned_on_timestamp: u64,
    timer_duration: Duration,
}

impl DeviceState {
    pub fn new() -> Self {
        Self {
            is_on: false,
            is_timer_set: false,
            last_turned_on_timestamp: 0,
            timer_duration: Duration::from_secs(0),
        }
    }

    pub fn turn_on(&mut self) {
        self.is_on = true;
        let start = SystemTime::now();
        let timestamp = start
            .duration_since(UNIX_EPOCH)
            .unwrap();
        self.last_turned_on_timestamp = timestamp.as_secs();
    }

    pub fn turn_off(&mut self) {
        self.is_on = false;
        self.last_turned_on_timestamp = 0;
    }
}
