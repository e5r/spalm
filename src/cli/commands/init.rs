use crate::cli::Cmd;

pub struct InitCommand {
    name: String
}

impl InitCommand {
    pub fn new() -> InitCommand {
        InitCommand {
            name: String::from("init")
        }
    }
}

impl Cmd for InitCommand {
    fn exec(&self) -> Result<(), (i32)> {
        println!("InitCommand #-> name: {}", self.name);
        Ok(())
    }
}