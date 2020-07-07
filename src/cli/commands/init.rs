use std::path::PathBuf;

use crate::cli::{CmdError, CmdHandler, CmdResult};

pub struct InitCommand {
    with_git: bool,
}

impl InitCommand {
    pub fn new(with_git: bool) -> InitCommand {
        InitCommand { with_git }
    }
}

impl CmdHandler for InitCommand {
    fn exec(&self, _path: PathBuf) -> CmdResult {
        Err(CmdError::UnknownError.into())
    }
}
