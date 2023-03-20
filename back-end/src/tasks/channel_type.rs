#[derive(Clone)]
pub struct PublishMessage {
    pub(super) topic: String,
    pub(super) message: Vec<u8>,
}

#[derive(Clone)]
pub struct TimerStartRequest {
    pub(super) is_turn_on: bool,
    pub(super) seconds_to_trigger: u64,
}

impl TimerStartRequest {
    pub fn new(is_turn_on: bool, seconds_to_trigger: u64) -> Self {
        Self {
            is_turn_on,
            seconds_to_trigger,
        }
    }
}
