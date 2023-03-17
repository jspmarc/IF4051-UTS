use super::{Connect, Disconnect};
use crate::entity::State;
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

impl Handler<Connect> for WsServer {
    type Result = ();

    fn handle(&mut self, _: Connect, _: &mut Self::Context) -> Self::Result {
        let counter = &self.app_state.counter;
        counter.fetch_add(1, Ordering::SeqCst);
        info!(
            "A session is CONNECTED | counter: {}",
            counter.load(Ordering::SeqCst)
        );
    }
}

impl Handler<Disconnect> for WsServer {
    type Result = ();

    fn handle(&mut self, _: Disconnect, _: &mut Self::Context) -> Self::Result {
        let counter = &self.app_state.counter;
        counter.fetch_sub(1, Ordering::SeqCst);
        info!(
            "A session is DISCONNECTED | counter: {}",
            counter.load(Ordering::SeqCst)
        );
    }
}
