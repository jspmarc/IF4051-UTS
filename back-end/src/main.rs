use actix_web::{web, Responder, get, HttpServer, App, HttpRequest, HttpResponse, http::StatusCode};
use actix_web_actors::ws;
use std::sync::{Mutex, Arc};
use entity::State;
use websocket::WsSession;

mod entity;
mod websocket;

#[get("/ws")]
async fn ws_handler(req: HttpRequest, stream: web::Payload, state: web::Data<State>) -> impl Responder {
    let state = state.into_inner();
    ws::start(WsSession::new(state), &req, stream)
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::new(StatusCode::NO_CONTENT)
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let state = web::Data::new(State {
        counter: Arc::new(Mutex::new(0)),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(hello)
            .service(ws_handler)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
