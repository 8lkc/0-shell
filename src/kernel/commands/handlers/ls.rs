use std::{fs::{self}, io, path::Path};

use crate::kernel::DIRECTORY_STACK;

use super::CommandHandler;

pub struct Ls;

impl CommandHandler for Ls {
    fn execute(&self, args: &[String]) -> Result<(), std::io::Error> {
        let mut all = false;
        let mut path = DIRECTORY_STACK::to_string();

        for arg in args {
            if arg.starts_with('-') {
                for ch in arg.chars().skip(1) {
                    match ch {
                        'a' => all = true,
                        _   => return Err(io::Error::new(
                            io::ErrorKind::InvalidInput,
                            format!("ls: unexpected argument '-{}' found", ch)
                        ))
                    }
                }
            } else {
                if arg.starts_with('/') {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("ls: cannot access '{}': No such file or directory", arg)
                    ));
                }
                path = format!("{}/{}", path, arg);
            }
        }

        Ls::listing(path, all)
    }
}

impl Ls {
    fn get_content<P: AsRef<Path>>(path: P, all: bool) -> Result<Vec<Item>, io::Error> {
        let mut items = Vec::new();
        let content = fs::read_dir(path)?;

        for entry in content {
            let entry = entry?;
            let name = entry.file_name();
            let name = name.to_string_lossy().to_string();

            if !all && name.starts_with('.') { continue; }
            items.push(Item { name });
        }

        Ok(items)
    }

    fn listing<P: AsRef<Path>>(path: P, all: bool) -> Result<(), io::Error> {
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

        Ok(())
    }
}

struct Item {
    name: String
}
