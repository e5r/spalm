pub trait Cmd {
    fn exec(&self) -> Result<(),(u32)>;
}