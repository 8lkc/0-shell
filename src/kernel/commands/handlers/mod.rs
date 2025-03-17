use std::io;

mod echo;
mod ls;
mod pwd;

pub(super) use {
    echo::Echo,
    ls::Ls,
    pwd::Pwd
};

pub trait CommandHandler {
    fn execute(&self, args: &[String]) -> Result<(), io::Error>;
}
