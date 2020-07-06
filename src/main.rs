use spalm::cli::Cmd;
use spalm::cli::commands::InitCommand;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name="spalm", version="0.1.0", about = "Specification Project of E5R Application Lifecycle Management")]
struct Opt {}

fn main() {
    // let init = InitCommand::new();

    // match init.exec() {
    //     Ok(_) => std::process::exit(0),
    //     Err(code) => std::process::exit(code)
    // }

    let opt = Opt::from_args();
    println!("{:?}", opt);
}