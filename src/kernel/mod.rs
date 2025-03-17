mod commands;

use std::{env, io::{self, Write}, path};

use commands::Command;

pub struct Shell;

impl Shell {
    pub fn launch() {
        loop {
            Self::show_prompt();

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("⚠️ Failed to read line");

            if input.trim() == "exit" {
                break;
            }

            input = input.trim().to_string();
            if !input.is_empty() {
                Command::check(&input);
            }
        }
    }

    fn show_prompt() {
        let dir = env!("CARGO_MANIFEST_DIR");
        let dir_path = path::Path::new(dir);
        let home = env::var("HOME").or_else(|_| env::var("USERPROFILE")).ok();

        if let Some(home) = home {
            let home_path = path::Path::new(&home);
            if let Ok(relative_path) = dir_path.strip_prefix(home_path) {
                print!("[~/{}]:~$ ", relative_path.display());
            } else {
                print!("[{}]:~$ ", dir);
            }
        } else {
            print!("[{}]:~$ ", dir);
        }

        // Flush to ensure the prompt is displayed immediately
        io::stdout().flush().expect("⚠️ Failed to flush prompt");
    }
}
