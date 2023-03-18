use serde::Serialize;

#[derive(Clone, Copy, Serialize)]
pub enum Device {
    Ac,
    Light,
}

#[derive(Clone, Copy, Serialize)]
pub struct DeviceStatus {
    is_on: bool,
    is_timer_set: bool,
    last_turned_on: u64,
    timer_start: u64,
}

impl DeviceStatus {
    pub fn new() -> Self {
        Self {
            is_on: false,
            is_timer_set: false,
            last_turned_on: 0,
            timer_start: 0,
        }
    }
}
