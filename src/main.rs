use structopt::StructOpt;

use spalm::cli::SpalmCli;

fn main() {
    match SpalmCli::from_args().run() {
        Ok(()) => std::process::exit(0),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1)
        }
    }
}
