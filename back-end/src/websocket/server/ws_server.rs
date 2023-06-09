use crate::{tasks::channel_type, websocket::State};
use actix::{Actor, Context};
use std::sync::Arc;
use tokio::sync::{broadcast, mpsc};

#[derive(Clone)]
pub struct WsServer {
    pub(super) app_state: Arc<State>,

    pub(super) tx_timer_ac: broadcast::Sender<channel_type::TimerStartRequest>,
    pub(super) tx_timer_light: broadcast::Sender<channel_type::TimerStartRequest>,
    pub(super) tx_mqtt_publisher: mpsc::Sender<channel_type::PublishMessage>,
}

impl WsServer {
    pub fn new(
        tx_timer_ac: broadcast::Sender<channel_type::TimerStartRequest>,
        tx_timer_light: broadcast::Sender<channel_type::TimerStartRequest>,
        tx_mqtt_publisher: mpsc::Sender<channel_type::PublishMessage>,
    ) -> Self {
        WsServer {
            app_state: Arc::new(State::new()),

            tx_timer_ac,
            tx_timer_light,
            tx_mqtt_publisher,
        }
    }
}

impl Actor for WsServer {
    type Context = Context<Self>;
}
