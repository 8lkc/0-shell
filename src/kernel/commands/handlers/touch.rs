use {
    super::{
        CommandHandler,
        MakeDirectory
    },
    crate::{
        kernel::DIRECTORY_STACK,
        Error,
        Tool
    },
    std::{
        fs::File,
        io
    }
};

pub struct Touch;

impl CommandHandler for Touch {
    fn execute(args: &[String]) -> Result<(), std::io::Error> {
        if !args.is_empty() {
            for arg in args {
                MakeDirectory::check_nesting("touch", &arg)?;

                if File::create(format!("{}/{}", DIRECTORY_STACK::to_string(), arg)).is_err() {
                    return Err(Error::throw(io::ErrorKind::HostUnreachable, &["touch:", arg]));
                }
            }

            Tool::push_to_history(&format!("touch {}", args.join(" ")))?;
            return Ok(());
        }

        Err(Error::throw(io::ErrorKind::NotSeekable, &["touch:"]))
    }
}
