use crate::{
    device_from_str,
    entity::{Device, Error},
    websocket::session::responses::{
        StatusResponse, SwitchResponse, TimerStartResponse, TimerStopResponse,
    },
};
use actix::Message;
use std::collections::HashSet;

macro_rules! split_str {
    ($s:expr, $delim:expr) => {
        $s.split($delim).collect::<Vec<&str>>()
    };

    ($s:expr, $delim:expr, unique) => {
        $s.split($delim).collect::<HashSet<&str>>()
    };
}

macro_rules! validate_args {
    ($args:expr, $expected_len:expr) => {
        if $args.len() != $expected_len {
            return Err(Error::ArgumentCountNotValid($expected_len, $args.len()));
        }
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
        validate_args!(args, 1);
        let args = split_str!(args[0], ':', unique);

        let mut devices: Vec<Device> = vec![];

        for device in args {
            let device = device_from_str!(device);
            devices.push(device);
        }

        Ok(Self { devices })
    }

    pub fn get_devices(&self) -> &Vec<Device> {
        &self.devices
    }
}

#[derive(Message)]
#[rtype(SwitchResponse)]
pub struct SwitchRequest {
    devices: Vec<Device>,
    is_turn_on: bool,
}

impl SwitchRequest {
    pub fn parse_args_string(args: &str) -> Result<Self, Error> {
        let args = split_str!(args, ' ');
        validate_args!(args, 2);
        let devices_args = split_str!(args[0], ':', unique);
        let is_turn_on = match args[1].to_lowercase().trim() {
            "on" => true,
            "off" => false,
            other => {
                return Err(Error::BadMessageWithReason(format!(
                    "expected: on | off; given: {}",
                    other
                )))
            }
        };

        let mut devices: Vec<Device> = vec![];

        for device in devices_args {
            let device = device_from_str!(device);
            devices.push(device);
        }

        Ok(Self {
            devices,
            is_turn_on,
        })
    }

    pub fn new(devices: Vec<Device>, is_turn_on: bool) -> Self {
        Self {
            devices,
            is_turn_on,
        }
    }

    pub fn get_devices(&self) -> &Vec<Device> {
        &self.devices
    }

    pub fn is_on(&self) -> bool {
        self.is_turn_on
    }
}

#[derive(Message)]
#[rtype(TimerStartResponse)]
pub struct TimerStartRequest {
    devices: Vec<Device>,
    is_turn_on: bool,
    seconds_to_trigger: u64,
}

impl TimerStartRequest {
    pub fn parse_args_string(args: &str) -> Result<Self, Error> {
        let args = split_str!(args, ' ');
        validate_args!(args, 3);
        let devices_args = split_str!(args[0], ':', unique);
        let is_turn_on = match args[1].to_lowercase().trim() {
            "on" => true,
            "off" => false,
            other => {
                return Err(Error::BadMessageWithReason(format!(
                    "expected: on | off; given: {}",
                    other
                )))
            }
        };
        let seconds_to_trigger = match args[2].parse::<u64>() {
            Ok(sec) => sec,
            Err(err) => return Err(Error::BadMessageWithReason(err.to_string())),
        };

        let mut devices: Vec<Device> = vec![];

        for device in devices_args {
            let device = device_from_str!(device);
            devices.push(device);
        }

        Ok(Self {
            devices,
            is_turn_on,
            seconds_to_trigger,
        })
    }

    pub fn get_devices(&self) -> &Vec<Device> {
        &self.devices
    }

    pub fn is_on(&self) -> bool {
        self.is_turn_on
    }

    pub fn seconds_to_trigger(&self) -> u64 {
        self.seconds_to_trigger
    }
}

#[derive(Message)]
#[rtype(TimerStopResponse)]
pub struct TimerStopRequest {
    devices: Vec<Device>,
}

impl TimerStopRequest {
    pub fn parse_args_string(args: &str) -> Result<Self, Error> {
        match StatusRequest::parse_args_string(args) {
            Ok(r) => Ok(Self { devices: r.devices }),
            Err(e) => Err(e),
        }
    }

    pub fn get_devices(&self) -> &Vec<Device> {
        &self.devices
    }
}

impl From<Device> for TimerStopRequest {
    fn from(value: Device) -> Self {
        Self {
            devices: vec![value],
        }
    }
}
