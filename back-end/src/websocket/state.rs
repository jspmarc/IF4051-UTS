use crate::entity::DeviceState;
use std::sync::{atomic::AtomicUsize, Arc, RwLock};

#[derive(Clone)]
#[allow(dead_code)]
pub struct State {
    pub counter: Arc<AtomicUsize>,

    ac: Arc<RwLock<DeviceState>>,
    light: Arc<RwLock<DeviceState>>,
}

impl State {
    pub fn new() -> Self {
        Self {
            // clients connected
            counter: Arc::new(AtomicUsize::new(0)),

            // device state
            ac: Arc::new(RwLock::new(DeviceState::new())),
            light: Arc::new(RwLock::new(DeviceState::new())),
        }
    }

    pub fn get_ac_state(&self) -> &RwLock<DeviceState> {
        self.ac.as_ref()
    }

    pub fn get_light_state(&self) -> &RwLock<DeviceState> {
        self.light.as_ref()
    }
}
