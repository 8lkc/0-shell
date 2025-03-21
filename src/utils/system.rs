use std::{io::{self, Write}, sync::{atomic::{AtomicBool, Ordering}, Arc}};

use termion::{input::TermRead, raw::IntoRawMode};

pub struct System;

impl System {
    pub fn add_io_listener<Func>(prompt: &str, handler: Func) -> Result<(), io::Error>
    where
        Func: Fn(&str) -> Result<(), io::Error> {

        let running = Arc::new(AtomicBool::new(true));
        let running_clone = running.clone();

        ctrlc::set_handler(move || {
            running_clone.store(false, Ordering::SeqCst);
        }).expect("Error setting Ctrl-C handler");

        let stdin = io::stdin();
        let mut stdout = io::stdout().into_raw_mode().unwrap();
        let mut stdin = stdin.lock().keys();
        let mut input = String::new();

        write!(stdout, "{}", prompt).unwrap();
        stdout.flush().unwrap();

        while running.load(Ordering::SeqCst) {
            if let Some(Ok(key)) = stdin.next() {
                match key {
                    termion::event::Key::Ctrl('c') => {
                        // running.store(false, Ordering::SeqCst);
                        write!(stdout, "\r\n{}", prompt).unwrap();
                        stdout.flush().unwrap();
                    }
                    termion::event::Key::Char(c) => {
                        if c == '\n' {
                            write!(stdout, "\r\n").unwrap();
                            if input == "exit" {
                                write!(stdout, "\r").unwrap();
                                running.store(false, Ordering::SeqCst);
                            } else {
                                let _ = handler(&input);
                                write!(stdout, "\r{}", prompt).unwrap();
                                input = String::new();
                            }
                        } else {
                            write!(stdout, "{}", c).unwrap();
                            input.push(c);
                        }
                        stdout.flush().unwrap();
                    }
                    termion::event::Key::Backspace => {
                        if !input.is_empty() {
                            input.pop();
                            write!(stdout, "\r{}{}", prompt, input).unwrap();
                            write!(stdout, " \r{}{}", prompt, input).unwrap(); // Clear the last character
                            stdout.flush().unwrap();
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }
}
