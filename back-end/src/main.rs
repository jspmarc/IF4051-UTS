use actix::{Actor, Addr};
use actix_web::{
    body::MessageBody, get, http::StatusCode, middleware::Logger, web, App, HttpRequest,
    HttpResponse, HttpServer, Responder,
};
use actix_web_actors::ws;
use log::{error, info, warn};
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
    std::env::set_var("RUST_LOG", "INFO");
    env_logger::init();

    // channels
    // channel to determine whether an async task should shutdown or not
    let (tx_shutdown, mut rx_shutdown) = mpsc::channel(1);
    let (tx_mqtt_publisher, rx_mqtt_publisher) = mpsc::channel::<channel_type::PublishMessage>(1);
    let (tx_timer_ac, _) = broadcast::channel::<channel_type::TimerStartRequest>(1);
    let (tx_timer_light, _) = broadcast::channel::<channel_type::TimerStartRequest>(1);

    // MQTT
    let host = match std::env::var("MQTT_HOST") {
        Ok(h) => h,
        Err(_) => {
            warn!("Can't find MQTT_HOST env variable, using the default 127.0.0.1");
            "127.0.0.1".to_string()
        }
    };
    let port = match std::env::var("MQTT_PORT") {
        Ok(h) => match h.parse::<u16>() {
            Ok(n) => n,
            Err(_) => {
                warn!("Can't parse MQTT_PORT env variable, using the default 1883");
                1883_u16
            }
        },
        Err(_) => {
            warn!("Can't find MQTT_PORT env variable, using the default 1883");
            1883_u16
        }
    };
    let mut mqtt_client = MqttClient::new(&host, port);
    if let Err(e) = mqtt_client.connect(3) {
        error!("{}", e.to_string());
        std::process::exit(1);
    }
    let client = match mqtt_client.get_client() {
        Ok(c) => c,
        Err(e) => panic!("{}", e.to_string()),
    };
    let _ = client.publish(
        paho_mqtt::MessageBuilder::new()
            .topic(mqtt::topic::MQTT_OUT_RESET_TOPIC)
            .payload("test".try_into_bytes().unwrap())
            .finalize(),
    );
    info!("Device state has been reset");
    let mqtt_publisher = task::spawn(mqtt_publisher(
        client.clone(),
        rx_mqtt_publisher,
        tx_shutdown.clone(),
    ));

    // tasks
    let task_timer_ac = tokio::spawn(task_timer(
        tx_timer_ac.clone(),
        tx_mqtt_publisher.clone(),
        tx_shutdown.clone(),
        entity::Device::Ac,
    ));
    let task_timer_light = tokio::spawn(task_timer(
        tx_timer_light.clone(),
        tx_mqtt_publisher.clone(),
        tx_shutdown.clone(),
        entity::Device::Light,
    ));

    // HTTP and WS server
    let host = match std::env::var("HTTP_HOST") {
        Ok(h) => h,
        Err(_) => {
            warn!("Can't find HTTP_HOST env variable, using the default 0.0.0.0");
            "0.0.0.0".to_string()
        }
    };
    let port = match std::env::var("HTTP_PORT") {
        Ok(h) => match h.parse::<u16>() {
            Ok(n) => n,
            Err(_) => {
                warn!("Can't parse HTTP_PORT env variable, using the default 8080");
                8080_u16
            }
        },
        Err(_) => {
            warn!("Can't find HTTP_PORT env variable, using the default 8080");
            8080_u16
        }
    };
    let ws_server = WsServer::new(tx_timer_ac, tx_timer_light, tx_mqtt_publisher).start();
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(ws_server.clone()))
            .wrap(Logger::default())
            .service(hello)
            .service(ws_handler)
    })
    .bind((host, port));
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
    let res = tokio::join!(
        server,
        mqtt_publisher,
        graceful_shutdown,
        task_timer_ac,
        task_timer_light,
    );

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
