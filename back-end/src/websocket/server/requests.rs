use crate::{
    entity::{Device, Error},
    websocket::responses::StatusResponse,
};
use actix::Message;

macro_rules! split_str {
    ($s:expr, $delim:expr) => {
        $s.split($delim).collect::<Vec<&str>>()
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
    pub fn parse_args_string(args: &str) -> Result<Self, Error> {
        let args = split_str!(args, ' ');
        if args.len() != 1 {
            return Err(Error::ArgumentCountNotValid(1, args.len()));
        }
        let args = split_str!(args[0], ':');

        let mut devices: Vec<Device> = vec![];

        for device in args {
            let device = match device.to_lowercase().as_str() {
                "ac" => Device::Ac,
                "light" => Device::Light,
                dev => return Err(Error::UnknownDevice(dev.to_string())),
            };

            devices.push(device);
        }

        Ok(Self { devices })
    }

    pub fn get_devices(&self) -> &Vec<Device> {
        &self.devices
    }
}
