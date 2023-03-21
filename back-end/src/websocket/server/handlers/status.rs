use crate::{
    entity::Device,
    websocket::{
        session::responses::{StatusResponse, StatusResponseElement},
        server::{requests::StatusRequest, WsServer},
    },
};
use actix::Handler;

impl WsServer {
    pub fn create_status_response(&mut self, msg: StatusRequest) -> StatusResponse {
        let state = &self.app_state;

        let mut response: StatusResponse = StatusResponse::default();

        let devices = msg.get_devices();

        for device in devices {
            let resp = match device {
                Device::Ac => state.get_ac_state(),
                Device::Light => state.get_light_state(),
            };
            let resp = resp.read().unwrap();
            let resp = StatusResponseElement::new(*device, *resp, None);

            response.push(resp);
        }

        response
    }
}

impl Handler<StatusRequest> for WsServer {
    type Result = StatusResponse;

    fn handle(&mut self, msg: StatusRequest, _: &mut Self::Context) -> Self::Result {
        self.create_status_response(msg)
    }
}
