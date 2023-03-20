use log::{error, info};
use std::{sync::Arc, thread, time::Duration};
use tokio::{
    sync::{
        broadcast::{self, error::TryRecvError},
        mpsc,
    },
    time,
};

use crate::mqtt;

mod channel_type;

pub fn mqtt_publisher(client: Arc<paho_mqtt::Client>, tx_shutdown: mpsc::Sender<()>) {
    let msg = paho_mqtt::MessageBuilder::new()
        .topic(mqtt::topic::MQTT_OUT_PING_TOPIC)
        .payload("")
        .finalize();
    while !tx_shutdown.is_closed() {
        let _ = client.publish(msg.clone());
        thread::sleep(Duration::from_secs(2));
    }

    info!("MQTT publisher stopped");
}

pub async fn task_timer(
    client: Arc<paho_mqtt::Client>,
    tx: broadcast::Sender<i32>,
    tx_shutdown: mpsc::Sender<()>,
) {
    let mut rx = tx.subscribe();
    info!("Timer task started");
    while !tx_shutdown.is_closed() {
        match rx.try_recv() {
            Ok(r) => {
                time::sleep(time::Duration::from_secs(r as u64)).await;
                info!("Timer triggered");
            }
            Err(e) => {
                if e != TryRecvError::Empty {
                    error!("Can't receive message from timer task channel {}", e);
                }
                time::sleep(time::Duration::from_millis(50)).await;
            }
        };
    }
    info!("Timer task stopped");
}
