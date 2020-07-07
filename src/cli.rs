mod cmd_error;
mod cmd_handler;
mod cmd_result;
mod commands;

pub use self::cmd_error::*;
pub use self::cmd_handler::*;
pub use self::cmd_result::*;

use std::path::PathBuf;
use structopt::StructOpt;

use crate::cli::commands::InitCommand;

#[derive(StructOpt)]
#[structopt(
    name = "spalm",
    version = "0.1.0",
    about = "Specification Project of E5R Application Lifecycle Management"
)]
pub struct SpalmCli {
    #[structopt(short, long, parse(from_os_str))]
    pub path: Option<PathBuf>,

    #[structopt(subcommand)]
    pub cmd: SpalmCmd,
}

#[derive(StructOpt)]
pub enum SpalmCmd {
    Init {
        #[structopt(short, long)]
        with_git: bool,
    },
}

impl SpalmCmd {
    pub fn resolve(&self) -> Box<dyn CmdHandler> {
        match self {
            SpalmCmd::Init { with_git } => Box::new(InitCommand::new(*with_git)),
        }
    }
}

impl SpalmCli {
    pub fn run(&self) -> CmdResult {
        self.cmd.resolve().exec(match &self.path {
            Some(p) => p.clone(),
            None => PathBuf::from("."),
        })
    }
}
