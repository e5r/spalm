use spalm::cli::Cmd;
use spalm::cli::commands::InitCommand;

fn main() {
    let init = InitCommand::new();

    match init.exec() {
        Ok(_) => std::process::exit(0),
        Err(code) => std::process::exit(code)
    }
}