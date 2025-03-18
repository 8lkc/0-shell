mod commands;
mod dir_stack;

use {
    std::{
        env,
        fs::File,
        io::{self, BufRead, Write},
        path
    },

    crate::rgb_to_ansi256,
    commands::Command,
    dir_stack::DIRECTORY_STACK
};

pub struct Shell;

impl Shell {
    pub fn launch() -> io::Result<()> {
        DIRECTORY_STACK::init();
        Self::show_header()?;

        loop {
            Self::show_prompt()?;

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("⛔ Failed to read line");

            if input.trim() == "exit" {
                println!();
                return Ok(());
            }

            input = input.trim().to_string();
            if !input.is_empty() {
                if let Err(err_msg) = Command::check(&input) { println!("⛔ {}", err_msg); }
            }
        }
    }

    fn show_header() -> io::Result<()> {
        println!("\n🚀 Welcome to . . .\x1b[0m\n");
        let file = File::open("assets/header.txt")?;
        let reader = io::BufReader::new(file);
        for line in reader.lines() {
            println!("\x1b[1;38;5;{}m{}\x1b[0m", rgb_to_ansi256(253, 240, 213), line?)
        }
        println!("\n🔥 Type '\x1b[1;3;38;5;{}mexit\x1b[0m' to quit the shell 😇\n", rgb_to_ansi256(46, 196, 182));
        Ok(())
    }

    fn show_prompt() -> Result<(), io::Error> {
        let dir = env!("CARGO_MANIFEST_DIR");
        let dir_path = path::Path::new(dir);
        let home = env::var("HOME").or_else(|_| env::var("USERPROFILE")).ok();

        if let Some(home) = home {
            let home_path = path::Path::new(&home);
            if let Ok(relative_path) = dir_path.strip_prefix(home_path) {
                print!(
                    "\x1b[1;38;5;{}m[~/{}]\x1b[38;5;{}m:\x1b[38;5;{}m{}\x1b[38;5;{}m$\x1b[0m ",
                    rgb_to_ansi256(143, 217, 73),
                    relative_path.display(),
                    rgb_to_ansi256(253, 254, 250),
                    rgb_to_ansi256(109, 156, 192),
                    DIRECTORY_STACK::to_string(),
                    rgb_to_ansi256(253, 254, 250)
                );
            } else { return Err(io::Error::new(io::ErrorKind::NotFound, "⛔ Prompt not found")) }
        } else { return Err(io::Error::new(io::ErrorKind::NotFound, "⛔ Prompt not found")) }

        // Flush to ensure the prompt is displayed immediately
        io::stdout().flush().expect("⚠️ Failed to flush prompt");
        Ok(())
    }
}
