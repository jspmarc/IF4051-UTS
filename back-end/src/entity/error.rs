pub enum Error {
    BadMessage,
    UnknownCommand(String),
}

impl ToString for Error {
    fn to_string(&self) -> String {
        match self {
            Self::BadMessage => "Can't process message".to_owned(),
            Self::UnknownCommand(cmd) => format!("Command {} is unknown", cmd),
        }
    }
}
