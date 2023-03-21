use crate::entity::{Device, DeviceState, Error};
use actix::{Message, MessageResponse};
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

#[derive(Default, Message, MessageResponse, Serialize)]
#[rtype(result = "()")]
pub struct StatusResponse {
    data: Vec<StatusResponseElement>,
}

impl StatusResponse {
    pub fn push(&mut self, el: StatusResponseElement) {
        self.data.push(el)
    }
}

pub type SwitchResponse = StatusResponse;

pub type TimerStartResponse = StatusResponse;

pub type TimerStopResponse = StatusResponse;
