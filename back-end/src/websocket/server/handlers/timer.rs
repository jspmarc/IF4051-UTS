use crate::{
    entity::{Device, Error},
    tasks::channel_type,
    websocket::{
        server::{
            requests::{TimerStartRequest, TimerStopRequest},
            WsServer,
        },
        session::responses::{StatusResponseElement, TimerStartResponse, TimerStopResponse},
    },
};
use actix::Handler;

impl Handler<TimerStartRequest> for WsServer {
    type Result = TimerStartResponse;

    fn handle(&mut self, msg: TimerStartRequest, _: &mut Self::Context) -> Self::Result {
        let state = &self.app_state;

        let mut response: TimerStartResponse = TimerStartResponse::default();

        let devices = msg.get_devices();
        let is_on = msg.is_on();
        let seconds_to_trigger = msg.seconds_to_trigger();

        for device in devices {
            let (resp, tx_timer) = match device {
                Device::Ac => (state.get_ac_state(), &self.tx_timer_ac),
                Device::Light => (state.get_light_state(), &self.tx_timer_light),
            };
            let mut device_state = resp.write().unwrap();
            let error = if device_state.is_timer_set() {
                Some(Error::TimerAlreadySet(device.to_string()))
            } else {
                device_state.set_timer(seconds_to_trigger, is_on);
                let msg = channel_type::TimerStartRequest::new(is_on, seconds_to_trigger);
                let _ = tx_timer.send(msg);
                None
            };

            let resp = StatusResponseElement::new(*device, *device_state, error);

            response.push(resp);
        }

        response
    }
}

impl Handler<TimerStopRequest> for WsServer {
    type Result = TimerStopResponse;

    fn handle(&mut self, msg: TimerStopRequest, _: &mut Self::Context) -> Self::Result {
        let state = &self.app_state;

        let mut response: TimerStopResponse = TimerStopResponse::default();

        let devices = msg.get_devices();

        for device in devices {
            let resp = match device {
                Device::Ac => state.get_ac_state(),
                Device::Light => state.get_light_state(),
            };
            let mut device_state = resp.write().unwrap();
            let error = if device_state.is_timer_set() {
                device_state.stop_timer();
                None
            } else {
                Some(Error::TimerNotSet(device.to_string()))
            };
            let resp = StatusResponseElement::new(*device, *device_state, error);

            response.push(resp);
        }

        response
    }
}
