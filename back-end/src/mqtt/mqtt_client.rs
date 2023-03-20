use log::info;

use crate::entity::Error;
use std::{sync::Arc, thread, time::Duration};

extern crate paho_mqtt as mqtt;

pub struct MqttClient {
    host: String,
    port: u16,
    mqtt_options: Option<mqtt::ConnectOptions>,
    client: Option<Arc<mqtt::Client>>,
    msg_stream: Option<mqtt::Receiver<Option<mqtt::Message>>>,
}

impl MqttClient {
    pub fn new(host: &str, port: u16) -> Self {
        Self {
            host: host.to_string(),
            port,
            mqtt_options: None,
            client: None,
            msg_stream: None,
        }
    }

    #[allow(dead_code)]
    pub fn set_options(&mut self, opts: &mqtt::ConnectOptions) {
        self.mqtt_options = Some(opts.clone());
    }

    pub fn connect(&mut self, max_retries: i32) -> Result<(), Error> {
        if self.is_connected() {
            return Ok(());
        }

        let default_opts = mqtt::ConnectOptionsBuilder::new()
            .keep_alive_interval(Duration::from_secs(30))
            .finalize();
        let opts = match &self.mqtt_options {
            Some(opt) => opt,
            None => &default_opts,
        };

        if self.client.is_none() {
            self.client = match mqtt::Client::new(format!("{}:{}", self.host, self.port)) {
                Ok(c) => Some(Arc::new(c)),
                Err(e) => return Err(Error::MqttClientCreationFailure(e.to_string())),
            };
        }
        let client = self.client.as_ref().unwrap();

        // start consuming
        if self.msg_stream.is_none() {
            self.msg_stream = Some(client.start_consuming());
        }

        info!("Connecting to MQTT broker...");
        if let Err(e) = client.connect(opts.clone()) {
            if max_retries == 0 {
                info!("Can't connect to MQTT broker, reason: {}", e);
                return Err(Error::MqttClientConnectionFailure(e.to_string()));
            }

            let mut retry_count = 0;
            thread::sleep(Duration::from_secs(1));

            while let Err(e) = client.reconnect() {
                retry_count += 1;
                if retry_count >= max_retries {
                    info!("Can't connect to MQTT broker, reason: {}", e);
                    return Err(Error::MqttClientConnectionFailure(e.to_string()));
                } else {
                    info!("Can't connect to MQTT broker, reason: {}. Retrying...", e);
                    thread::sleep(Duration::from_secs(1 * retry_count as u64));
                }
            }
        }
        info!("Connected to MQTT broker");

        Ok(())
    }

    pub fn get_client(&self) -> Result<Arc<mqtt::Client>, Error> {
        match &self.client {
            Some(c) => Ok(c.clone()),
            None => Err(Error::MqttClientNotCreated),
        }
    }

    pub fn is_connected(&self) -> bool {
        self.client.is_some() && self.client.as_ref().map(|x| x.is_connected()).unwrap()
    }
}
