use std::io;

mod echo;
mod ls;
mod pwd;
mod cd;
mod history;
mod mkdir;
mod touch;

pub(super) use {
    echo::Echo,
    history::History,
    ls::List,
    pwd::PrintWorkingDirectory,
    touch::Touch
};
pub use {
    cd::ChangeDirectory,
    mkdir::MakeDirectory
};

pub trait CommandHandler {
    fn execute(args: &[String]) -> Result<(), io::Error>;
}
