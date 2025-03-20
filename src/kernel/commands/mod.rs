mod handlers;

use {
    crate::{
        command,
        kernel::commands::handlers::CommandHandler,
        Error
    },
    handlers::{
    //     Concatenate,
        ChangeDirectory,
        Echo,
        History,
        List,
        MakeDirectory,
        PrintWorkingDirectory,
       Touch
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
            "mkdir"    => MakeDirectory::execute($arguments),
            "touch"    => Touch::execute($arguments),
            // "cat"      => Concatenate::execute($arguments),
            _ => Err(Error::throw(io::ErrorKind::InvalidInput, &[$command]))
        }
    };
}
