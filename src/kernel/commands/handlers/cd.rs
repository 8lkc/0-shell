use {
    super::CommandHandler,
    crate::{
        kernel::DIRECTORY_STACK,
        Error,
        Tool
    },
    std::{fs, io}
};

pub struct ChangeDirectory;

impl CommandHandler for ChangeDirectory {
    fn execute(args: &[String]) -> Result<(), std::io::Error> {
        if args.is_empty() {
            DIRECTORY_STACK::set_from(vec!["~".to_string()]);
            Tool::push_to_history("cd")?;
            return Ok(());
        } else if args.len() > 1 {
            return Err(Error::throw(io::ErrorKind::ArgumentListTooLong, &["cd:"]));
        }

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
                        return Err(Error::throw(io::ErrorKind::NotFound, &["cd:", part]));
                    }
                }
            }
        }

        DIRECTORY_STACK::set_from(current_stack);
        Tool::push_to_history(&format!("cd {}", args[0]))?;
        Ok(())
    }
}

impl ChangeDirectory {
    pub fn is_subdir(parent: &str, child: &str) -> bool {
        let content = fs::read_dir(parent).unwrap();
        for entry in content {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() && entry.file_name().into_string().unwrap() == child { return true; }
        }
        false
    }
}
