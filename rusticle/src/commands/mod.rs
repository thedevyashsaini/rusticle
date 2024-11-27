pub mod execute;
pub mod reject;
pub mod install;

pub trait Command {
    fn execute(&self);
}