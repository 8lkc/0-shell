use std::{io::{self, Write}, sync::{atomic::{AtomicBool, Ordering}, Arc}};

use termion::{input::TermRead, raw::IntoRawMode};

pub struct System;

impl System {
    pub fn add_io_listener<Func>(prompt_getter: Option<fn() -> Result<String, io::Error>>, handler: Func) -> Result<(), io::Error>
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

        if let Some(get_prompt) = prompt_getter {
            write!(stdout, "\r{}", get_prompt()?).unwrap();
            stdout.flush().unwrap();
        }

        while running.load(Ordering::SeqCst) {
            if let Some(Ok(key)) = stdin.next() {
                match key {
                    termion::event::Key::Ctrl('c') => {
                        if let Some(get_prompt) = prompt_getter {
                            write!(stdout, "\r\n{}", get_prompt()?).unwrap();
                            stdout.flush().unwrap();
                        } else {
                            running.store(false, Ordering::SeqCst);
                        }
                    }
                    termion::event::Key::Char(character) => {
                        match character {
                            '\n' => {
                                writeln!(stdout, "\r").unwrap();
                                if input == "exit" && !prompt_getter.is_none() {
                                    write!(stdout, "\r\n").unwrap();
                                    running.store(false, Ordering::SeqCst);
                                } else {
                                    if !input.is_empty() {
                                        let _ = handler(&input);
                                    }
                                    if let Some(get_prompt) = prompt_getter {
                                        write!(stdout, "\r{}", get_prompt()?).unwrap();
                                        stdout.flush().unwrap();
                                    }
                                    input.clear();
                                }
                            } _ => {
                                write!(stdout, "{}", character).unwrap();
                                input.push(character);
                            }
                        }
                        stdout.flush().unwrap();
                    }
                    termion::event::Key::Backspace => {
                        if !input.is_empty() {
                            input.pop();
                            let prompt = if let Some(get_prompt) = prompt_getter {
                                get_prompt()?
                            } else { String::new() };
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
