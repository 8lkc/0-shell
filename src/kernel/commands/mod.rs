mod handlers;

use std::io;

use handlers::Echo;

use crate::{
    command,
    kernel::commands::handlers::CommandHandler
};
// use crate::;

pub(super) struct Command;

impl Command {
    pub(super) fn check(input: &str) -> Result<(), io::Error> {
        let input: Vec<&str> = input.split_whitespace().collect();

        let args: Vec<String> = input.iter().skip(1)
                                     .map(|str| str.to_string())
                                     .collect();
        command!(input[0], &args)
    }
}

#[macro_export]
macro_rules! command {
    ($command:expr, $arguments:expr) => {
        match $command {
            "echo" => Echo.execute($arguments),
            _      => Err(std::io::Error::new(std::io::ErrorKind::Other, format!("Command '{}' not found", $command)))
        }
    };
}
