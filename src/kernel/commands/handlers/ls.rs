use {
    super::CommandHandler,
    crate::{
        kernel::DIRECTORY_STACK,
        Error,
        Tool
    },
    std::{
        fs::{self},
        io,
        path::Path
    }
};

pub struct List;

impl CommandHandler for List {
    fn execute(args: &[String]) -> Result<(), std::io::Error> {
        let mut all = false;
        let mut path = DIRECTORY_STACK::to_string();

        for arg in args {
            if arg.starts_with('-') {
                for ch in arg.chars().skip(1) {
                    match ch {
                        'a' => all = true,
                        _   => return Err(Error::throw(io::ErrorKind::UnexpectedEof, &["ls:", &ch.to_string()]))
                    }
                }
            } else {
                if arg.starts_with('/') {
                    return Err(Error::throw(io::ErrorKind::NotFound, &["ls:", arg]));
                }
                path = format!("{}/{}", path, arg);
            }
        }

        List::listing(path, all, args)
    }
}

impl List {
    fn get_content<P: AsRef<Path>>(path: P, all: bool) -> Result<Vec<Item>, io::Error> {
        let mut items = Vec::new();
        let content = match fs::read_dir(&path) {
            Ok(content) => content,
            Err(_) => {
                return Err(Error::throw(io::ErrorKind::NotFound, &["ls:", path.as_ref().to_str().unwrap()]));
            }
        };

        for entry in content {
            let entry = entry?;
            let name = entry.file_name();
            let name = name.to_string_lossy().to_string();

            if !all && name.starts_with('.') { continue; }
            items.push(Item { name });
        }

        Ok(items)
    }

    fn listing<P: AsRef<Path>>(path: P, all: bool, args: &[String]) -> Result<(), io::Error> {
        let items = Self::get_content(path, all)?;

        if all { print!(" .   .."); }
        if !items.is_empty() {
            if all { print!("   "); }
            print!("{}", items.first().unwrap().name);
            for item in items.iter().skip(1) {
                print!("   {}", item.name);
            }
            println!();
        }

        Tool::push_to_history(&format!("ls {}", args.join(" ")))?;
        Ok(())
    }
}

struct Item {
    name: String
}
