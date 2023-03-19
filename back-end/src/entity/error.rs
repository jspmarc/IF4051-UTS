pub enum Error {
    BadMessage,
    BadMessageWithReason(String),
    ArgumentCountNotValid(usize, usize),
    TimerAlreadySet(String),
    UnknownCommand(String),
    UnknownDevice(String),
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
            Self::UnknownCommand(cmd) => format!("Command {} is unknown", cmd),
            Self::UnknownDevice(dev) => format!("Device {} is unknown", dev),
        }
    }
}
