use crate::commands::Command;

pub struct Invalid {
    pub message: String,
}

impl Command for Invalid {
    fn execute(&self) {
        eprintln!("{}", self.message);
    }
}