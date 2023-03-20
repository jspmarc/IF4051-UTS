use self::channel_type::{PublishMessage, TimerStartRequest};
use crate::{entity::Device, mqtt};
use log::{error, info};
use std::sync::Arc;
use tokio::{
    sync::{broadcast, mpsc},
    time,
};

pub mod channel_type;

pub async fn mqtt_publisher(
    client: Arc<paho_mqtt::Client>,
    mut rx: mpsc::Receiver<PublishMessage>,
    tx_shutdown: mpsc::Sender<()>,
) {
    while !tx_shutdown.is_closed() {
        match rx.try_recv() {
            Ok(msg) => {
                let topic = msg.topic.clone();
                let msg = paho_mqtt::MessageBuilder::new()
                    .topic(msg.topic)
                    .payload(msg.message)
                    .finalize();
                let _ = client.publish(msg);
                info!("Published a message to MQTT topic {}", topic);
            }
            Err(e) => {
                if e != mpsc::error::TryRecvError::Empty {
                    error!("Can't receive message from timer task channel {}", e);
                }
            }
        };
        time::sleep(time::Duration::from_millis(50)).await;
    }

    info!("MQTT publisher stopped");
}

pub async fn task_timer(
    tx: broadcast::Sender<TimerStartRequest>,
    tx_publish_mqtt: mpsc::Sender<PublishMessage>,
    tx_shutdown: mpsc::Sender<()>,
    device: Device,
) {
    let mut rx = tx.subscribe();
    info!("Timer task started");
    let topic = match device {
        Device::Ac => mqtt::topic::MQTT_OUT_AC_TOPIC,
        Device::Light => mqtt::topic::MQTT_OUT_LIGHT_TOPIC,
    };
    while !tx_shutdown.is_closed() {
        match rx.try_recv() {
            Ok(msg) => {
                info!("Got timer request. Wait duration: {}", msg.seconds_to_trigger);
                time::sleep(time::Duration::from_secs(msg.seconds_to_trigger)).await;
                info!("Timer triggered");
                let _ = tx_publish_mqtt.send(PublishMessage {
                    topic: topic.to_string(),
                    message: vec![msg.is_turn_on.into()],
                }).await;
            }
            Err(e) => {
                if e != broadcast::error::TryRecvError::Empty {
                    error!("Can't receive message from timer task channel {}", e);
                }
                time::sleep(time::Duration::from_millis(50)).await;
            }
        };
    }
    info!("Timer task stopped");
}
