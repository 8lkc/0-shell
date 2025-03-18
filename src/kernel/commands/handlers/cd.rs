use {
    super::CommandHandler,
    crate::{
        kernel::DIRECTORY_STACK,
        rgb_to_ansi256
    },
    std::{fs, io}
};

pub struct ChangeDirectory;

impl CommandHandler for ChangeDirectory {
    fn execute(args: &[String]) -> Result<(), std::io::Error> {
        let mut current_path = DIRECTORY_STACK::to_string();
        let mut current_stack = DIRECTORY_STACK::get_copy();
        let arg_parts = args[0].split("/").collect::<Vec<&str>>();

        for part in arg_parts {
            match part {
                "." => continue,
                ".." => {
                    if current_stack.len() <= 1 { continue; }
                    current_stack.pop();
                    current_path = current_stack.join("/");
                },
                _ => {
                    if ChangeDirectory::is_subdir(&current_path, part) {
                        current_stack.push(part.to_string());
                        current_path = current_stack.join("/");
                    } else {
                        return Err(io::Error::new(
                            io::ErrorKind::Other,
                            format!(
                                "\x1b[1;3mcd:\x1b[0m \x1b[38;5;{}mno such directory:\x1b[0m \x1b[1;38;5;{}m{}\x1b[0m",
                                rgb_to_ansi256(220, 45, 34), rgb_to_ansi256(251, 177, 60), part
                            )
                        ));
                    }
                }
            }
        }

        DIRECTORY_STACK::set_from_vec(current_stack);
        Ok(())
    }
}

impl ChangeDirectory {
    fn is_subdir(parent: &str, child: &str) -> bool {
        let content = fs::read_dir(parent).unwrap();
        for entry in content {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() && entry.file_name().into_string().unwrap() == child { return true; }
        }
        false
    }
}
