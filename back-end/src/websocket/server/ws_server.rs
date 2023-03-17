use super::requests::{ConnectRequest, DisconnectRequest, StatusRequest};
use crate::entity::{Device, State};
use actix::{Actor, Context, Handler};
use log::info;
use std::sync::{atomic::Ordering, Arc};

#[derive(Clone)]
pub struct WsServer {
    app_state: Arc<State>,
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

impl Handler<ConnectRequest> for WsServer {
    type Result = ();

    fn handle(&mut self, _: ConnectRequest, _: &mut Self::Context) -> Self::Result {
        let counter = &self.app_state.counter;
        counter.fetch_add(1, Ordering::SeqCst);
        info!(
            "A session is CONNECTED | counter: {}",
            counter.load(Ordering::SeqCst)
        );
    }
}

impl Handler<DisconnectRequest> for WsServer {
    type Result = ();

    fn handle(&mut self, _: DisconnectRequest, _: &mut Self::Context) -> Self::Result {
        let counter = &self.app_state.counter;
        counter.fetch_sub(1, Ordering::SeqCst);
        info!(
            "A session is DISCONNECTED | counter: {}",
            counter.load(Ordering::SeqCst)
        );
    }
}

impl Handler<StatusRequest> for WsServer {
    type Result = bool;

    fn handle(&mut self, msg: StatusRequest, _: &mut Self::Context) -> Self::Result {
        let state = &self.app_state;
        match msg.get_device() {
            Device::Ac => state.is_ac_on().load(Ordering::SeqCst),
            Device::Light => state.is_light_on().load(Ordering::SeqCst),
            _ => false,
        }
    }
}
