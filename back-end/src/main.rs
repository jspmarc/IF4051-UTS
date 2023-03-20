use actix::{Actor, Addr};
use actix_web::{
    get, http::StatusCode, middleware::Logger, web, App, HttpRequest, HttpResponse, HttpServer,
    Responder,
};
use actix_web_actors::ws;
use log::{error, info};
use mqtt::MqttClient;
use tasks::*;
use tokio::{
    signal,
    sync::{broadcast, mpsc},
    task,
};
use websocket::{server::WsServer, session::WsSession};

mod entity;
mod mqtt;
mod tasks;
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
    // ::std::env::set_var("RUST_LOG", "actix_web=debug,INFO");
    ::std::env::set_var("RUST_LOG", "INFO");
    env_logger::init();

    // channel to determine whether an async task should shutdown or not
    let (tx_shutdown, mut rx_shutdown) = mpsc::channel(1);
    let (tx_timer, _) = broadcast::channel::<i32>(1);

    // MQTT
    let mut mqtt_client = MqttClient::new("127.0.0.1", 1883);
    if let Err(e) = mqtt_client.connect(3) {
        error!("{}", e.to_string());
        std::process::exit(1);
    }
    let client = match mqtt_client.get_client() {
        Ok(c) => c,
        Err(e) => panic!("{}", e.to_string()),
    };
    let client_clone = client.clone();
    let tx_shutdown_clone = tx_shutdown.clone();
    let mqtt_publisher =
        task::spawn_blocking(move || mqtt_publisher(client_clone, tx_shutdown_clone));

    // tasks
    let task_timer = tokio::spawn(task_timer(
        client.clone(),
        tx_timer.clone(),
        tx_shutdown.clone(),
    ));

    // HTTP and WS server
    let ws_server = WsServer::new(&tx_timer).start();
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(ws_server.clone()))
            .wrap(Logger::default())
            .service(hello)
            .service(ws_handler)
    })
    .bind(("0.0.0.0", 8080));
    if let Err(e) = server {
        error!("{}", e);
        rx_shutdown.close();
        std::process::exit(1);
    }
    let server = server.unwrap().run();

    let graceful_shutdown = task::spawn(async move {
        match signal::ctrl_c().await {
            Ok(()) => (),
            Err(err) => {
                error!("Unable to listen for shutdown signal: {}", err);
                // we also shut down in case of error
            }
        }
        info!("Closing shutdown channel...");
        rx_shutdown.close();
        info!("Shutdown channel closed");
    });

    // join tasks
    let res = tokio::join!(server, mqtt_publisher, graceful_shutdown, task_timer);

    if client.is_connected() {
        let _ = client.disconnect(None);
        info!("MQTT client disconnected");
    }

    if let Err(e) = res.0 {
        panic!("{}", e);
    }

    if let Err(e) = res.1 {
        panic!("{}", e);
    }

    Ok(())
}
