use std::io;

mod echo;

pub(super) use echo::Echo;

pub trait CommandHandler {
    fn execute(&self, args: &[String]) -> Result<(), io::Error>;
}
