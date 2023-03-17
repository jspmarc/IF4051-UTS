use crate::entity::constants;
use crate::websocket::server::WsServer;
use actix::{
    fut, Actor, ActorContext, ActorFutureExt, Addr, AsyncContext, ContextFutureSpawner,
    StreamHandler, WrapFuture,
};
use actix_web_actors::ws;
use log::{error, info};
use std::time::Instant;

use super::server;

type WsResult = Result<ws::Message, ws::ProtocolError>;

pub struct WsSession {
    server: Addr<WsServer>,
    heartbeat_instant: Instant,
}

impl WsSession {
    pub fn new(server: &Addr<WsServer>) -> Self {
        WsSession {
            server: server.clone(),
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

        self.server
            .send(server::Connect {})
            .into_actor(self)
            .then(|res, _act, ctx| {
                match res {
                    Ok(_) => (),
                    _ => {
                        error!("Can't connect to server");
                        ctx.stop();
                    }
                };

                fut::ready(())
            })
            .wait(ctx)
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        self.server.do_send(server::Disconnect {})
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
