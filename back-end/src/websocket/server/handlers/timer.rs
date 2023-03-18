use crate::{websocket::{
    server::{requests::TimerRequest, WsServer},
    session::responses::{TimerResponse, StatusResponseElement},
}, entity::Device};
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
            // TODO: add check whether timer is set or not, if set then return error
            device_state.set_timer(timer_trigger_timestamp, is_on);
            let resp = StatusResponseElement::new(*device, *device_state);

            response.push(resp);
        }

        response
    }
}
