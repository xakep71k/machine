pub trait Machine {}
pub trait Command {
    fn execute(&self) -> (bool, usize);
    fn size(&self) -> usize;
}
