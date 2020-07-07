use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub enum CmdError {
    UnknownError,
}

impl fmt::Display for CmdError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::UnknownError => "unknown error!",
            }
        )
    }
}

impl Error for CmdError {}
