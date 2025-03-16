use std::{env, io::{self, Write}, path};

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
                println!();
                break;
            }

            if !input.trim().is_empty() {
                println!("Command '{}' not found", input.trim());
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
