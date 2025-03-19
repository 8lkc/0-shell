use {
    std::{
        io::{self, Write},
        sync::{
            atomic::{
                AtomicBool,
                Ordering
            },
            Arc
        }
    },
    termion::{
        input::TermRead,
        raw::IntoRawMode
    }
};

use super::CommandHandler;

pub struct Concatenate;

impl CommandHandler for Concatenate {
    fn execute(args: &[String]) -> Result<(), std::io::Error> {
        if args.is_empty() {
            // Create an AtomicBool wrapped in an Arc to share between threads
            let running = Arc::new(AtomicBool::new(true));
            // Clone the Arc to use in the signal handler
            let running_clone = running.clone();

            // Set up the Ctrl+C handler
            ctrlc::set_handler(move || {
                // Set the running flag to false when Ctrl+C is pressed
                running_clone.store(false, Ordering::SeqCst);
            }).expect("Error setting Ctrl-C handler");

            // Enable raw mode to read input without waiting for Enter
            let stdin = io::stdin();
            let mut stdout = io::stdout().into_raw_mode().unwrap();
            let mut stdin = stdin.lock().keys();

            // Main loop that runs while the running flag is true
            while running.load(Ordering::SeqCst) {
                if let Some(Ok(key)) = stdin.next() {
                    match key {
                        termion::event::Key::Ctrl('c') => {
                            running.store(false, Ordering::SeqCst);
                        }
                        termion::event::Key::Char(c) => {
                            if c == '\n' {
                                write!(stdout, "\r\n").unwrap();
                            } else {
                                write!(stdout, "{}", c).unwrap();
                            }
                            stdout.flush().unwrap();
                        }
                        _ => {}
                    }
                }
            }
            println!("\nReceived Ctrl+C, exiting...\r");
        }

        Ok(())
    }
}
