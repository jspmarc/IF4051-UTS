use crate::entity::State;
use actix::{Actor, Context};
use std::sync::Arc;

#[derive(Clone)]
pub struct WsServer {
    pub(super) app_state: Arc<State>,
}

impl WsServer {
    pub fn new() -> Self {
        WsServer {
            app_state: Arc::new(State::new()),
        }
    }
}

impl Actor for WsServer {
    type Context = Context<Self>;
}
