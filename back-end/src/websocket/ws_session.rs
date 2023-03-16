use crate::entity::{constants, State};
use actix::{Actor, ActorContext, AsyncContext, StreamHandler};
use actix_web_actors::ws;
use log::info;
use std::{sync::Arc, time::Instant};

type WsResult = Result<ws::Message, ws::ProtocolError>;

pub struct WsSession {
    app_state: Arc<State>,
    heartbeat_instant: Instant,
}

impl WsSession {
    pub fn new(state: Arc<State>) -> Self {
        WsSession {
            app_state: state.clone(),
            heartbeat_instant: Instant::now(),
        }
    }

    pub fn start_heartbeat(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(constants::WS_HEARTBEAT_INTERVAL, |actor, ctx| {
            let hb = actor.heartbeat_instant;
            if Instant::now().duration_since(hb) > constants::WS_CLIENT_TIMEOUT {
                return ctx.stop();
            }

            ctx.ping(&[]);
        });
    }
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.start_heartbeat(ctx);

        let mut counter = self.app_state.counter.lock().unwrap();
        *counter += 1;

        info!("A session is STARTED | counter: {}", counter);
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        let mut counter = self.app_state.counter.lock().unwrap();
        *counter -= 1;

        info!("A session is STOPPED | counter: {}", counter);
    }
}

impl StreamHandler<WsResult> for WsSession {
    fn handle(&mut self, msg: WsResult, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.heartbeat_instant = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.heartbeat_instant = Instant::now();
            }
            Ok(ws::Message::Text(text)) => {
                info!("Got message: {}", text.trim());
                ctx.text(text);
            }
            // ignore continuation, binary, and nop messages
            Ok(ws::Message::Continuation(_))
            | Ok(ws::Message::Nop)
            | Ok(ws::Message::Binary(_)) => (),
            _ => ctx.stop(),
        }
    }
}
