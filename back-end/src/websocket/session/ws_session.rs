use crate::entity::{constants, Error};
use crate::websocket::server::{requests::StatusRequest, WsServer};
use actix::{
    fut, Actor, ActorContext, ActorFutureExt, Addr, AsyncContext, ContextFutureSpawner,
    StreamHandler, WrapFuture,
};
use actix_web_actors::ws;
use log::{error, info};
use std::time::Instant;

use crate::websocket::server;

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
}

impl WsSession {
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
            .send(server::requests::ConnectRequest {})
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
        self.server.do_send(server::requests::DisconnectRequest {})
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
                let text = text.trim();
                let server = &self.server;
                match text.split_once(' ') {
                    // status [device]
                    // [device]: ac | light | :[device]
                    Some(("status", args)) => {
                        info!("Got topic status | args: {:?}", args);
                        let msg = match StatusRequest::from_string(args) {
                            Ok(msg) => msg,
                            Err(err) => return ctx.text(err.to_string()),
                        };
                        server
                            .send(msg)
                            .into_actor(self)
                            .then(|res, _act, ctx| {
                                match res {
                                    Ok(res) => ctx.text(serde_json::to_string(&res).unwrap()),
                                    Err(_) => {
                                        error!("Can't send message to server");
                                        ctx.stop();
                                    }
                                };

                                fut::ready(())
                            })
                            .wait(ctx)
                    }
                    // status [device]:[state]
                    // [device]: ac | light | :[device]
                    // [state]: on | off
                    Some(("switch", args)) => {
                        info!("Got topic switch | args: {:?}", args);
                        ctx.text(args);
                    }
                    // timer [device]:[state]:[time]
                    // [device]: ac | light | :[device]
                    // [state]: on | off
                    // [time]: [number]
                    // [number]: how many seconds until turned on or off
                    Some(("timer", args)) => {
                        info!("Got topic timer");
                        ctx.text(args);
                    }
                    Some((cmd, _)) => ctx.text(Error::UnknownCommand(cmd.to_owned()).to_string()),
                    _ => ctx.text(Error::BadMessage.to_string()),
                }
            }
            // ignore continuation, binary, and nop messages
            Ok(ws::Message::Continuation(_))
            | Ok(ws::Message::Nop)
            | Ok(ws::Message::Binary(_)) => (),
            _ => ctx.stop(),
        }
    }
}
