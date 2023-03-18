pub enum Error {
    BadMessage,
    ArgumentCountNotValid(usize, usize),
    UnknownCommand(String),
    UnknownDevice(String),
}

impl ToString for Error {
    fn to_string(&self) -> String {
        match self {
            Self::BadMessage => "Can't process message".to_owned(),
            Self::ArgumentCountNotValid(expected, given) => {
                format!("Expected {} argument(s), given {}", expected, given)
            }
            Self::UnknownCommand(cmd) => format!("Command {} is unknown", cmd),
            Self::UnknownDevice(dev) => format!("Device {} is unknown", dev),
        }
    }
}
