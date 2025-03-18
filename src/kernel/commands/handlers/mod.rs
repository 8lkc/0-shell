use std::io;

mod echo;
mod ls;
mod pwd;
mod cd;

pub(super) use {
    echo::Echo,
    ls::List,
    pwd::PrintWorkingDirectory
};

pub trait CommandHandler {
    fn execute(args: &[String]) -> Result<(), io::Error>;
}
