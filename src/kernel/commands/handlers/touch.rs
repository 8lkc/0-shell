use {
    super::{
        CommandHandler,
        MakeDirectory
    },
    crate::{
        kernel::DIRECTORY_STACK,
        Util
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
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput, 
                        format!(
                            "\x1b[1;3mtouch:\x1b[0m cannot create directory '\x1b[1;38;5;{}m{}\x1b[0m': \x1b[38;5;{}mFile exists\x1b[0m",
                            Util::rgb_to_ansi256(251, 177, 60), arg, Util::rgb_to_ansi256(220, 45, 34)
                        )
                    ));
                }
            }

            Util::push_to_history(&format!("touch {}", args.join(" ")))?;
            return Ok(());
        }

        Err(io::Error::new(
            io::ErrorKind::InvalidInput, 
            format!("\x1b[1;3mtouch:\x1b[0m \x1b[38;5;{}mmissing operand\x1b[0m", Util::rgb_to_ansi256(220, 45, 34))
        ))
    }
}
