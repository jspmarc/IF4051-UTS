use std::sync::{atomic::AtomicU32, Arc, RwLock};

use super::DeviceStatus;

#[derive(Clone)]
#[allow(dead_code)]
pub struct State {
    pub counter: Arc<AtomicU32>,

    ac: Arc<RwLock<DeviceStatus>>,
    light: Arc<RwLock<DeviceStatus>>,
}

impl State {
    pub fn new() -> Self {
        Self {
            // clients connected
            counter: Arc::new(AtomicU32::new(0)),

            // device state
            ac: Arc::new(RwLock::new(DeviceStatus::new())),
            light: Arc::new(RwLock::new(DeviceStatus::new())),
        }
    }

    pub fn get_ac_state(&self) -> &RwLock<DeviceStatus> {
        self.ac.as_ref()
    }

    pub fn get_light_state(&self) -> &RwLock<DeviceStatus> {
        self.light.as_ref()
    }
}
