use crate::websocket::server::{
    requests::{ConnectRequest, DisconnectRequest, StatusRequest},
    WsServer,
};
use actix::Handler;
use log::info;
use std::sync::atomic::Ordering;

impl Handler<ConnectRequest> for WsServer {
    type Result = ();

    fn handle(&mut self, msg: ConnectRequest, _: &mut Self::Context) -> Self::Result {
        let counter = &self.app_state.counter;
        counter.fetch_add(1, Ordering::SeqCst);
        info!(
            "A session is CONNECTED | counter: {}",
            counter.load(Ordering::SeqCst)
        );

        msg.recipient.do_send(self.create_status_response(StatusRequest::default()));
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
