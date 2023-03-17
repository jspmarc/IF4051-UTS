use crate::entity::{Device, Error};
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
#[rtype(bool)]
pub struct StatusRequest {
    device: Device,
}

impl StatusRequest {
    pub fn from_string(s: &str) -> Result<Self, Error> {
        let s = split_str!(s);

        if s.len() != 1 {
            return Err(Error::BadMessage);
        }

        let device = match s[0].to_lowercase().as_str() {
            "ac" => Device::Ac,
            "light" => Device::Light,
            "all" => Device::All,
            _ => return Err(Error::BadMessage),
        };

        Ok(Self { device })
    }

    pub fn get_device(&self) -> &Device {
        &self.device
    }
}
