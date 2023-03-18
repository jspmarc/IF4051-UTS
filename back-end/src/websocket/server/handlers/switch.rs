use crate::{
    entity::Device,
    websocket::{
        session::responses::{StatusResponseElement, SwitchResponse},
        server::{requests::SwitchRequest, WsServer},
    },
};
use actix::Handler;

impl Handler<SwitchRequest> for WsServer {
    type Result = SwitchResponse;

    fn handle(&mut self, msg: SwitchRequest, _: &mut Self::Context) -> Self::Result {
        let state = &self.app_state;

        let mut response: SwitchResponse = vec![];

        let devices = msg.get_devices();

        for device in devices {
            let resp = match device {
                Device::Ac => state.get_ac_state(),
                Device::Light => state.get_light_state(),
            };
            let mut device_state = resp.write().unwrap();
            if msg.is_on() {
                device_state.turn_on()
            } else {
                device_state.turn_off()
            }
            // TODO: add interfacing with MQTT here
            let resp = StatusResponseElement::new(*device, *device_state);

            response.push(resp);
        }

        response
    }
}
