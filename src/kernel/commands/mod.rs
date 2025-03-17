mod handlers;

use std::io;

use crate::execute;

pub(super) struct Command;

impl Command {
    pub(super) fn check(input: &str) -> io::Result<()> {
        let input: Vec<&str> = input.split_whitespace().collect();

        execute!(input[0], input.iter().skip(1));
        Ok(())
    }
}

#[macro_export]
macro_rules! execute {
    ($command:expr, $args:expr) => {
        match $command {
            _ => println!("⛔ Command '{}' not found", $command)
        }
    };
}
