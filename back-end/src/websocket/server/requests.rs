use crate::{
    entity::{Device, Error},
    websocket::session::responses::{StatusResponse, SwitchResponse, TimerResponse}, device_from_str,
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

    pub fn get_devices(&self) -> &Vec<Device> {
        &self.devices
    }

    pub fn is_on(&self) -> bool {
        self.is_turn_on
    }
}

#[derive(Message)]
#[rtype(TimerResponse)]
pub struct TimerRequest {
    devices: Vec<Device>,
    is_turn_on: bool,
    timer_trigger_timestamp: u64,
}

impl TimerRequest {
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
        let timer_trigger_timestamp = match args[2].parse::<u64>() {
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
            timer_trigger_timestamp,
        })
    }

    pub fn get_devices(&self) -> &Vec<Device> {
        &self.devices
    }

    pub fn is_on(&self) -> bool {
        self.is_turn_on
    }

    pub fn get_timer_trigger_timestamp(&self) -> u64 {
        self.timer_trigger_timestamp
    }
}
