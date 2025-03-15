use dirs;
use std::io::{self, Write};

pub struct Shell;

impl Shell {
    pub fn launch() {
        loop {
            Self::show_prompt();

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("⚠️ Failed to read line");

            if input.trim() == "exit" { break; }

            if !input.trim().is_empty() {
                println!("_> {}", input.trim());
            }
        }
    }

    fn show_prompt() {
        let home = dirs::home_dir().expect("Could not find home directory");
        let dir = env!("CARGO_MANIFEST_DIR");
        let dir_path = std::path::Path::new(dir);

        if let Ok(relative_path) = dir_path.strip_prefix(&home) {
            print!("[~/{}]:~$ ", relative_path.display());
        } else {
            print!("[~/{}]:~$ ", dir);
        }
        // Flush to ensure the prompt is displayed immediately
        io::stdout().flush().expect("⚠️ Failed to flush stdout");
    }
}
