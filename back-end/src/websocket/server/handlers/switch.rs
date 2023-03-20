use crate::{
    entity::Device,
    mqtt,
    tasks::channel_type,
    websocket::{
        server::{requests::SwitchRequest, WsServer},
        session::responses::{StatusResponseElement, SwitchResponse},
    },
};
use actix::Handler;

impl Handler<SwitchRequest> for WsServer {
    type Result = SwitchResponse;

    fn handle(&mut self, msg: SwitchRequest, _: &mut Self::Context) -> Self::Result {
        let state = &self.app_state;

        let mut response: SwitchResponse = vec![];

        let devices = msg.get_devices();
        let is_on = msg.is_on();

        for device in devices {
            let (resp, topic) = match device {
                Device::Ac => (state.get_ac_state(), mqtt::topic::MQTT_OUT_AC_TOPIC),
                Device::Light => (state.get_light_state(), mqtt::topic::MQTT_OUT_LIGHT_TOPIC),
            };
            let mut device_state = resp.write().unwrap();
            let msg_payload: u8 = if is_on {
                device_state.turn_on();
                1
            } else {
                device_state.turn_off();
                0
            };
            let tx_pub = self.tx_mqtt_publisher.clone();
            tokio::task::spawn_blocking(move || {
                let msg = channel_type::PublishMessage::new(topic.to_string(), vec![msg_payload]);
                let _ = tx_pub.blocking_send(msg);
            });
            let resp = StatusResponseElement::new(*device, *device_state, None);

            response.push(resp);
        }

        response
    }
}
