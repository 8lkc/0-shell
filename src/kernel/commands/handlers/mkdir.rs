use {
    super::{ChangeDirectory, CommandHandler}, crate::{kernel::DIRECTORY_STACK, push_to_history, rgb_to_ansi256}, std::{fs, io}
};

pub struct MakeDirectory;

impl CommandHandler for MakeDirectory {
    fn execute(args: &[String]) -> Result<(), io::Error> {
        if !args.is_empty() {
            for arg in args {
                let parts = arg.split("/").collect::<Vec<&str>>();
                println!("{:?}", parts);
                if parts.len() > 1 {
                    if ChangeDirectory::is_subdir(&DIRECTORY_STACK::to_string(), parts[0]) {
                        return Ok(());
                    }

                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput, 
                        format!(
                            "\x1b[1;3mmkdir:\x1b[0m cannot create directory '\x1b[1;38;5;{}m{}\x1b[0m': \x1b[38;5;{}mNo such file or directory\x1b[0m",
                            rgb_to_ansi256(251, 177, 60), arg, rgb_to_ansi256(220, 45, 34)
                        )
                    ));
                }

                println!("{:?}", arg);
                if fs::create_dir(format!("{}/{}", DIRECTORY_STACK::to_string(), arg)).is_err() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput, 
                        format!(
                            "\x1b[1;3mmkdir:\x1b[0m cannot create directory '\x1b[1;38;5;{}m{}\x1b[0m': \x1b[38;5;{}mFile exists\x1b[0m",
                            rgb_to_ansi256(251, 177, 60), arg, rgb_to_ansi256(220, 45, 34)
                        )
                    ));
                }
            }

            push_to_history(&format!("mkdir {}", args.join(" ")))?;
            return Ok(());
        }

        Err(io::Error::new(
            io::ErrorKind::InvalidInput, 
            format!("\x1b[1;3mmkdir:\x1b[0m \x1b[38;5;{}mmissing operand\x1b[0m", rgb_to_ansi256(220, 45, 34))
        ))
    }
}
