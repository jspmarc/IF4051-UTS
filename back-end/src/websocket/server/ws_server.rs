use super::requests::{ConnectRequest, DisconnectRequest, StatusRequest};
use crate::{
    entity::{Device, State},
    websocket::responses::{StatusResponse, StatusResponseElement},
};
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
    type Result = StatusResponse;

    fn handle(&mut self, msg: StatusRequest, _: &mut Self::Context) -> Self::Result {
        let state = &self.app_state;

        let mut response: StatusResponse = vec![];

        let devices = msg.get_devices();

        for device in devices {
            let resp = match device {
                Device::Ac => state.get_ac_state(),
                Device::Light => state.get_light_state(),
            };
            let resp = resp.read().unwrap();
            let resp = StatusResponseElement::new(*device, *resp);

            response.push(resp);
        }

        response
    }
}
