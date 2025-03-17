mod handlers;

use crate::execute;

pub(super) struct Command;

impl Command {
    pub(super) fn check(input: &str) {
        execute!(input, vec![]);
    }
}

#[macro_export]
macro_rules! execute {
    ($command:expr, $args:expr) => {
        match $command {
            _ => println!("⚠️ Command '{}' not found", $command)
        }
    };
}
