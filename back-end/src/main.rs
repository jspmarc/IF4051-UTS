use actix::{Addr, Actor};
use actix_web::{
    get, http::StatusCode, middleware::Logger, web, App, HttpRequest, HttpResponse, HttpServer,
    Responder,
};
use actix_web_actors::ws;
use websocket::{server::WsServer, WsSession};

mod entity;
mod websocket;

#[get("/ws")]
async fn ws_handler(
    req: HttpRequest,
    stream: web::Payload,
    server: web::Data<Addr<WsServer>>,
) -> impl Responder {
    ws::start(WsSession::new(&server), &req, stream)
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::new(StatusCode::NO_CONTENT)
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    ::std::env::set_var("RUST_LOG", "actix_web=debug,INFO");
    env_logger::init();

    let ws_server = WsServer::new().start();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(ws_server.clone()))
            .wrap(Logger::default())
            .service(hello)
            .service(ws_handler)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
