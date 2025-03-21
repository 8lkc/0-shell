mod commands;
mod dir_stack;

use {
    crate::{System, Tool},
    commands::Command,
    dir_stack::DIRECTORY_STACK,
    std::{env, io::{self, Write}, path}
};

pub struct Shell;

impl Shell {
    pub fn launch() -> Result<(), io::Error> {
        DIRECTORY_STACK::init();
        Tool::show_header()?;

        // loop {
            System::add_io_listener(&Self::get_prompt()?, |input| {
                Ok(println!("{}", input))
            })?;

// System::add_io_listener(|input| {
//     // Define the specific operation here
//     println!("You entered: {}", input);
// })?;

            Ok(())

            // let mut input = String::new();
            // io::stdin()
            //     .read_line(&mut input)
            //     .expect("⛔ Failed to read line");
    
            // if input.trim() == "exit" {
            //     println!();
            //     Tool::push_to_history("exit")?;
            //     return Ok(());
            // }
    
            // input = input.trim().to_string();
            // if !input.is_empty() {
            //     if let Err(err_msg) = Command::check(&input) { println!("⛔ {}", err_msg); }
            // }
        // }
    }

    fn get_prompt() -> Result<String, io::Error> {
        let dir = env!("CARGO_MANIFEST_DIR");
        let dir_path = path::Path::new(dir);
        let home = env::var("HOME").or_else(|_| env::var("USERPROFILE")).ok();
        let mut output = String::new();

        if let Some(home) = home {
            let home_path = path::Path::new(&home);
            if let Ok(relative_path) = dir_path.strip_prefix(home_path) {
                output.push_str(&format!("{} ",
                    Tool::boldify(&format!("{}{}{}{}",
                        Tool::colorize(&format!("[~/{}]", relative_path.display()), (143, 217, 73)),
                        Tool::colorize(":", (253, 254, 250)),
                        Tool::colorize(&DIRECTORY_STACK::to_string(), (109, 156, 192)),
                        Tool::colorize("$", (253, 254, 250))
                    ))
                ));
            } else { return Err(io::Error::new(io::ErrorKind::InvalidInput, "⛔ Prompt not found")) }
        } else { return Err(io::Error::new(io::ErrorKind::InvalidInput, "⛔ Prompt not found")) }

        Ok(output)
    }
}
