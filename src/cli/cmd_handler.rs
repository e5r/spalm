use std::path::PathBuf;

use crate::cli::CmdResult;

pub trait CmdHandler {
    fn exec(&self, path: PathBuf) -> CmdResult;
}
