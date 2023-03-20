use crate::entity::State;
use actix::{Actor, Context};
use tokio::sync::broadcast;
use std::sync::Arc;

#[derive(Clone)]
pub struct WsServer {
    pub(super) app_state: Arc<State>,
    pub(super) tx_timer: broadcast::Sender<i32>,
}

impl WsServer {
    pub fn new(tx_timer: &broadcast::Sender<i32>) -> Self {
        WsServer {
            app_state: Arc::new(State::new()),
            tx_timer: tx_timer.clone(),
        }
    }
}

impl Actor for WsServer {
    type Context = Context<Self>;
}
