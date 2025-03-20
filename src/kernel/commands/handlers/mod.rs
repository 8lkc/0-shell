mod echo;
mod ls;
mod pwd;
mod cd;
mod history;
mod mkdir;
mod touch;
// mod cat;

pub(super) use {
//     cat::Concatenate,
    cd::ChangeDirectory,
    echo::Echo,
    history::History,
    ls::List,
    mkdir::MakeDirectory,
    pwd::PrintWorkingDirectory,
    touch::Touch
};
use std::io;

pub trait CommandHandler {
    fn execute(args: &[String]) -> Result<(), io::Error>;
}
