mod handlers;

use {
    crate::{
        command,
        kernel::commands::handlers::CommandHandler
    },
    handlers::{
        ChangeDirectory,
        Echo,
        History,
        List,
        PrintWorkingDirectory
    },
    std::io
};

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
            "echo"     => Echo::execute($arguments),
            "ls"       => List::execute($arguments),
            "pwd"      => PrintWorkingDirectory::execute($arguments),
            "cd"       => ChangeDirectory::execute($arguments),
            "history"  => History::execute($arguments),
            _ => Err(io::Error::new(io::ErrorKind::Other, format!("Command '{}' not found", $command)))
        }
    };
}
