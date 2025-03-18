use std::io;

mod echo;
mod ls;
mod pwd;
mod cd;
mod history;

pub(super) use {
    cd::ChangeDirectory,
    echo::Echo,
    history::History,
    ls::List,
    pwd::PrintWorkingDirectory
};

pub trait CommandHandler {
    fn execute(args: &[String]) -> Result<(), io::Error>;
}
