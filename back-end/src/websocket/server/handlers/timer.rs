use crate::{
    entity::{Device, Error},
    websocket::{
        server::{requests::TimerRequest, WsServer},
        session::responses::{StatusResponseElement, TimerResponse},
    },
};
use actix::Handler;

impl Handler<TimerRequest> for WsServer {
    type Result = TimerResponse;

    fn handle(&mut self, msg: TimerRequest, _: &mut Self::Context) -> Self::Result {
        let state = &self.app_state;

        let mut response: TimerResponse = vec![];

        let devices = msg.get_devices();
        let is_on = msg.is_on();
        let timer_trigger_timestamp = msg.get_timer_trigger_timestamp();

        for device in devices {
            let resp = match device {
                Device::Ac => state.get_ac_state(),
                Device::Light => state.get_light_state(),
            };
            let mut device_state = resp.write().unwrap();
            let error = if device_state.is_timer_set() {
                Some(Error::TimerAlreadySet(device.to_string()))
            } else {
                device_state.set_timer(timer_trigger_timestamp, is_on);
                None
            };
            let resp = StatusResponseElement::new(*device, *device_state, error);

            response.push(resp);
        }

        response
    }
}
