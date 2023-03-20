use crate::{entity::State, tasks::channel_type};
use actix::{Actor, Context};
use std::sync::Arc;
use tokio::sync::broadcast;

#[derive(Clone)]
pub struct WsServer {
    pub(super) app_state: Arc<State>,
    pub(super) tx_timer_ac: broadcast::Sender<channel_type::TimerStartRequest>,
    pub(super) tx_timer_light: broadcast::Sender<channel_type::TimerStartRequest>,
}

impl WsServer {
    pub fn new(
        tx_timer_ac: &broadcast::Sender<channel_type::TimerStartRequest>,
        tx_timer_light: &broadcast::Sender<channel_type::TimerStartRequest>,
    ) -> Self {
        WsServer {
            app_state: Arc::new(State::new()),
            tx_timer_ac: tx_timer_ac.clone(),
            tx_timer_light: tx_timer_light.clone(),
        }
    }
}

impl Actor for WsServer {
    type Context = Context<Self>;
}
