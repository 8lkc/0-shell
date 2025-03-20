use {
    super::CommandHandler,
    crate::{
        kernel::DIRECTORY_STACK,
        Error,
        Tool
    },
    std::io
};

pub struct PrintWorkingDirectory;

impl CommandHandler for PrintWorkingDirectory {
    fn execute(args: &[String]) -> Result<(), std::io::Error> {
        if args.is_empty() {
            println!("/01-shell{}", DIRECTORY_STACK::to_string().chars().skip(1).collect::<String>());
            Tool::push_to_history("pwd")?;
            return Ok(());
        }
        Err(Error::throw(io::ErrorKind::ArgumentListTooLong, &["pwd:"]))
    }
}
