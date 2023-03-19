use crate::entity::{Device, DeviceState, Error};
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusResponseElement {
    name: Device,
    status: DeviceState,
    error: Option<String>,
}

impl StatusResponseElement {
    pub fn new(name: Device, status: DeviceState, error: Option<Error>) -> Self {
        Self {
            name,
            status,
            error: error.map(|err| err.to_string()),
        }
    }
}

pub type StatusResponse = Vec<StatusResponseElement>;

pub type SwitchResponse = StatusResponse;

pub type TimerStartResponse = StatusResponse;

pub type TimerStopResponse = StatusResponse;
