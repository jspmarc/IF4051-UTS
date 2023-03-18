use serde::Serialize;

use crate::entity::{Device, DeviceState};

#[derive(Serialize)]
pub struct StatusResponseElement {
    name: Device,
    status: DeviceState,
}

impl StatusResponseElement {
    pub fn new(name: Device, status: DeviceState) -> Self {
        Self { name, status }
    }
}

pub type StatusResponse = Vec<StatusResponseElement>;

pub type SwitchResponse = StatusResponse;

pub type TimerResponse = StatusResponse;
