pub enum Error {
    BadMessage,
    BadMessageWithReason(String),
    ArgumentCountNotValid(usize, usize),
    TimerAlreadySet(String),
    TimerNotSet(String),
    UnknownCommand(String),
    UnknownDevice(String),

    MqttClientNotCreated,
    MqttClientCreationFailure(String),
    MqttClientConnectionFailure(String),
}

impl ToString for Error {
    fn to_string(&self) -> String {
        match self {
            Self::BadMessage => "Can't process message".to_owned(),
            Self::BadMessageWithReason(reason) => {
                format!("Can't process message because: {}", reason)
            }
            Self::ArgumentCountNotValid(expected, given) => {
                format!("Expected {} argument(s), given {}", expected, given)
            }
            Self::TimerAlreadySet(dev) => format!(
                "Can't set timer for device {} because it is already set",
                dev
            ),
            Self::TimerNotSet(dev) => format!(
                "Can't stop timer for device {} because it is not yet set",
                dev
            ),
            Self::UnknownCommand(cmd) => format!("Command {} is unknown", cmd),
            Self::UnknownDevice(dev) => format!("Device {} is unknown", dev),

            Self::MqttClientNotCreated => "MQTT client is not yet created".to_string(),
            Self::MqttClientCreationFailure(msg) => {
                format!("Can't create MQTT client. Reason: {}", msg)
            }
            Self::MqttClientConnectionFailure(msg) => {
                format!("Can't connect to MQTT broker. Reason: {}", msg)
            }
        }
    }
}
