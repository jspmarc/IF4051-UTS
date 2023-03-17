use std::{
    sync::{
        atomic::{AtomicBool, AtomicU32},
        Arc, Mutex,
    },
    time::Instant,
};

#[derive(Clone)]
#[allow(dead_code)]
pub struct State {
    pub counter: Arc<AtomicU32>,

    is_ac_on: Arc<AtomicBool>,
    ac_last_turned_on: Arc<Mutex<Instant>>,
    is_ac_timer_set: Arc<AtomicBool>,
    ac_timer: Arc<Mutex<Instant>>,

    is_light_on: Arc<AtomicBool>,
    light_last_turned_on: Arc<Mutex<Instant>>,
    is_light_timer_set: Arc<AtomicBool>,
    light_timer: Arc<Mutex<Instant>>,
}

impl State {
    pub fn new() -> Self {
        let now = Instant::now();

        Self {
            // clients connected
            counter: Arc::new(AtomicU32::new(0)),

            // device state
            is_ac_on: Arc::new(AtomicBool::new(false)),
            ac_last_turned_on: Arc::new(Mutex::new(now)),
            is_ac_timer_set: Arc::new(AtomicBool::new(false)),
            ac_timer: Arc::new(Mutex::new(now)),

            is_light_on: Arc::new(AtomicBool::new(false)),
            light_last_turned_on: Arc::new(Mutex::new(now)),
            is_light_timer_set: Arc::new(AtomicBool::new(false)),
            light_timer: Arc::new(Mutex::new(now)),
        }
    }

    pub fn is_ac_on(&self) -> &AtomicBool {
        self.is_ac_on.as_ref()
    }

    pub fn is_light_on(&self) -> &AtomicBool {
        self.is_light_on.as_ref()
    }
}
