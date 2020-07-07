use std::error::Error;

pub type CmdResult = Result<(), Box<dyn Error>>;
