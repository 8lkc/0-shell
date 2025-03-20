use {
    super::{
        ChangeDirectory,
        CommandHandler
    },
    crate::{
        kernel::DIRECTORY_STACK, Error, Tool
    },
    std::{fs, io}
};

pub struct MakeDirectory;

impl CommandHandler for MakeDirectory {
    fn execute(args: &[String]) -> Result<(), io::Error> {
        if !args.is_empty() {
            for arg in args {
                MakeDirectory::check_nesting("mkdir", &arg)?;

                if fs::create_dir(format!("{}/{}", DIRECTORY_STACK::to_string(), arg)).is_err() {
                    return Err(Error::throw(io::ErrorKind::AlreadyExists, &["mkdir:", arg]));
                }
            }

            Tool::push_to_history(&format!("mkdir {}", args.join(" ")))?;
            return Ok(());
        }

        Err(Error::throw(io::ErrorKind::NotSeekable, &["mkdir:"]))
    }
}

impl MakeDirectory {
    pub fn check_nesting(command: &str, argument: &str) -> Result<(), io::Error> {
        let parts = argument.split("/").collect::<Vec<&str>>();
        if parts.len() > 1 {
            if !ChangeDirectory::is_subdir(&DIRECTORY_STACK::to_string(), parts[0]) {
                return Err(Error::throw(io::ErrorKind::HostUnreachable, &[command, argument]));
            }
            
            // TO-DO: '-p' option
            return Ok(());
        }
        Ok(())
    }
}
