use log::error;
use std::env;
use std::process;

use spalm::cli::commands::InitCommand;
use spalm::cli::Cmd;

fn main() {
    pretty_env_logger::init();

    let args: Vec<String> = env::args().skip(1).collect();
    let cmd_result = match args.split_first() {
        Some((cmd, args)) if cmd.to_lowercase() == "init" => {
            if args.len() < 1 {
                error!("Invalid arguments for init command");
                InitCommand::usage();
                process::exit(2);
            }

            InitCommand::new(args.to_vec()).exec()
        }
        _ => {
            error!("No commands entered");
            usage();
            Err(1)
        }
    };

    match cmd_result {
        Ok(_) => process::exit(0),
        Err(code) => process::exit(code),
    }
}

fn usage() {
    let exe_name = spalm::utils::get_exe_name();

    println!("Usage: {} COMMAND [args]", exe_name);
}
