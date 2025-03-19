use {
    super::CommandHandler,
    crate::{
        kernel::DIRECTORY_STACK,
        Util
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
                        _   => return Err(io::Error::new(
                            io::ErrorKind::InvalidInput,
                            format!(
                                "\x1b[1;3mls:\x1b[0m unexpected argument '\x1b[1;3;38;5;{}m-{}\x1b[0m' found",
                                Util::rgb_to_ansi256(251, 177, 60) , ch
                            )
                        ))
                    }
                }
            } else {
                if arg.starts_with('/') {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        format!(
                            "\x1b[1;3mls:\x1b[0m cannot access '\x1b[1;38;5;{}m{}\x1b[0m': \x1b[38;5;{}mNo such file or directory\x1b[0m",
                            Util::rgb_to_ansi256(251, 177, 60), arg, Util::rgb_to_ansi256(220, 45, 34)
                        )
                    ));
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
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!(
                        "\x1b[1;3mls:\x1b[0m cannot access '\x1b[1;38;5;{}m{}\x1b[0m': \x1b[38;5;{}mNo such file or directory\x1b[0m",
                        Util::rgb_to_ansi256(251, 177, 60), path.as_ref().display(), Util::rgb_to_ansi256(220, 45, 34)
                    )
                ));
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

        Util::push_to_history(&format!("ls {}", args.join(" ")))?;
        Ok(())
    }
}

struct Item {
    name: String
}
