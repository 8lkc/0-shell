use std::io;

mod echo;
mod ls;
mod pwd;
mod cd;
mod history;
mod mkdir;

pub(super) use {
    echo::Echo,
    history::History,
    ls::List,
    mkdir::MakeDirectory,
    pwd::PrintWorkingDirectory
};
pub use cd::ChangeDirectory;

pub trait CommandHandler {
    fn execute(args: &[String]) -> Result<(), io::Error>;
}
