use std::io;

use crate::kernel::DIRECTORY_STACK;

use super::CommandHandler;

pub struct PrintWorkingDirectory;

impl CommandHandler for PrintWorkingDirectory {
    fn execute(args: &[String]) -> Result<(), std::io::Error> {
        if args.is_empty() {
            println!("/01-shell{}", DIRECTORY_STACK::to_string().chars().skip(1).collect::<String>());
            return Ok(());
        }
        Err(io::Error::new(io::ErrorKind::InvalidInput, "pwd: too many arguments"))
    }
}
