use {
    std::io,

    crate::{
        kernel::DIRECTORY_STACK,
        rgb_to_ansi256
    },
    super::CommandHandler
};

pub struct PrintWorkingDirectory;

impl CommandHandler for PrintWorkingDirectory {
    fn execute(args: &[String]) -> Result<(), std::io::Error> {
        if args.is_empty() {
            println!("/01-shell{}", DIRECTORY_STACK::to_string().chars().skip(1).collect::<String>());
            return Ok(());
        }
        Err(io::Error::new(
            io::ErrorKind::InvalidInput, 
            format!("\x1b[1;3mpwd:\x1b[0m \x1b[38;5;{}mtoo many arguments\x1b[0m", rgb_to_ansi256(220, 45, 34))
        ))
    }
}
