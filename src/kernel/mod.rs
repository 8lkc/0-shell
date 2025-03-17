mod commands;

use std::{env, fs::File, io::{self, BufRead, Write}, path};

use commands::Command;

pub struct Shell;

impl Shell {
    pub fn launch() -> io::Result<()> {
        Self::show_header()?;
        loop {
            Self::show_prompt();

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("⚠️ Failed to read line");

            if input.trim() == "exit" {
                println!();
                return Ok(());
            }

            input = input.trim().to_string();
            if !input.is_empty() { Command::check(&input)?; }
        }
    }

    fn show_header() -> io::Result<()> {
        println!("\n🚀 Welcome to . . .\n");
        let file = File::open("assets/header.txt")?;
        let reader = io::BufReader::new(file);
        for line in reader.lines() { println!("{}", line?) }
        println!("\n🔥 Type 'exit' to quit the shell 😇\n");
        Ok(())
    }

    fn show_prompt() {
        let dir = env!("CARGO_MANIFEST_DIR");
        let dir_path = path::Path::new(dir);
        let home = env::var("HOME").or_else(|_| env::var("USERPROFILE")).ok();

        if let Some(home) = home {
            let home_path = path::Path::new(&home);
            if let Ok(relative_path) = dir_path.strip_prefix(home_path) {
                print!("[~/{}]:~$ ", relative_path.display());
            } else { print!("[{}]:~$ ", dir); }
        } else { print!("[{}]:~$ ", dir); }

        // Flush to ensure the prompt is displayed immediately
        io::stdout().flush().expect("⚠️ Failed to flush prompt");
    }
}
