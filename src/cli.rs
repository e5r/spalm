pub mod commands;

pub trait Cmd {
    fn exec(self) -> Result<(), (i32)>;
}
