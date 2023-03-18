use serde::Serialize;

use crate::entity::{DeviceStatus, Device};

#[derive(Serialize)]
pub struct StatusResponseElement {
    name: Device,
    status: DeviceStatus,
}

impl StatusResponseElement {
    pub fn new(name: Device, status: DeviceStatus) -> Self {
        Self {
            name, status
        }
    }
}

pub type StatusResponse = Vec<StatusResponseElement>;
