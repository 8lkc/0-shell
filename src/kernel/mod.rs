mod commands;
mod dir_stack;

use {
    crate::Tool,
    commands::Command,
    dir_stack::DIRECTORY_STACK,
    std::{
        env,
        io::{self, BufRead, Write},
        path
    }
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
                Tool::push_to_history("exit")?;
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
        let reader = Tool::read_file("assets/header.txt")?;
        for line in reader.lines() { println!("{}", Tool::boldify(&Tool::colorize(&line?, (253, 240, 213)))) }
        println!("\n🔥 Type '{}' to quit the shell 😇\n", Tool::boldify(&Tool::italicify(&Tool::colorize("exit", (46, 196, 182)))));
        Ok(())
    }

    fn show_prompt() -> Result<(), io::Error> {
        let dir = env!("CARGO_MANIFEST_DIR");
        let dir_path = path::Path::new(dir);
        let home = env::var("HOME").or_else(|_| env::var("USERPROFILE")).ok();

        if let Some(home) = home {
            let home_path = path::Path::new(&home);
            if let Ok(relative_path) = dir_path.strip_prefix(home_path) {
                print!("{} ",
                    Tool::boldify(&format!("{}{}{}{}",
                        Tool::colorize(&format!("[~/{}]", relative_path.display()), (143, 217, 73)),
                        Tool::colorize(":", (253, 254, 250)),
                        Tool::colorize(&DIRECTORY_STACK::to_string(), (109, 156, 192)),
                        Tool::colorize("$", (253, 254, 250))
                    ))
                );
            } else { return Err(io::Error::new(io::ErrorKind::InvalidInput, "⛔ Prompt not found")) }
        } else { return Err(io::Error::new(io::ErrorKind::InvalidInput, "⛔ Prompt not found")) }

        // Flush to ensure the prompt is displayed immediately
        io::stdout().flush().expect("⚠️ Failed to flush prompt");
        Ok(())
    }
}
