use crate::{
    entity::{Device, Error},
    websocket::responses::StatusResponse,
};
use actix::Message;

macro_rules! split_str {
    ($s:ident) => {
        $s.split(':').collect::<Vec<&str>>()
    };
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ConnectRequest;

#[derive(Message)]
#[rtype(result = "()")]
pub struct DisconnectRequest;

#[derive(Message)]
#[rtype(StatusResponse)]
pub struct StatusRequest {
    devices: Vec<Device>,
}

impl StatusRequest {
    pub fn from_string(s: &str) -> Result<Self, Error> {
        let s = split_str!(s);

        let mut devices: Vec<Device> = vec![];

        for device in s {
            let device = match device.to_lowercase().as_str() {
                "ac" => Device::Ac,
                "light" => Device::Light,
                _ => return Err(Error::BadMessage),
            };

            devices.push(device);
        }

        Ok(Self { devices })
    }

    pub fn get_devices(&self) -> &Vec<Device> {
        &self.devices
    }
}
