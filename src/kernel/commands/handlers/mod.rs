use std::io;

mod echo;
mod ls;

pub(super) use {
    echo::Echo,
    ls::Ls
};

pub trait CommandHandler {
    fn execute(&self, args: &[String]) -> Result<(), io::Error>;
}
