use std::io::{self, Write};

use super::CommandHandler;

pub struct Echo;

impl CommandHandler for Echo {
    fn execute(&self, args: &[String]) -> Result<(), std::io::Error> {
        let mut args = args.to_vec();
        let mut opened_quote = String::new();

        if let Some(first) = args.first_mut() {
            if first.starts_with('"') || first.starts_with('\'') {
                opened_quote = first.chars().next().unwrap().to_string();
                first.remove(0);
            }
        }

        if let Some(last) = args.last_mut() {
            let mut input = String::new();
            if !opened_quote.is_empty() {
                let mut miss_closed_quote = false;

                if opened_quote == "'" {
                    if last.ends_with('\'') {
                        last.pop();
                    } else {
                        miss_closed_quote = true;
                    }
                } else {
                    if last.ends_with('"') {
                        last.pop();
                    } else {
                        miss_closed_quote = true
                    }
                }

                if miss_closed_quote {
                    while input.trim() != opened_quote {
                        input.clear();
                        print!("{}quote> ", if opened_quote == "'" { "" } else { "d" });
                        io::stdout().flush().expect("⚠️ Failed to flush prompt");
                        io::stdin()
                            .read_line(&mut input)
                            .expect("⚠️ Failed to read line");
                    }
                }
            }
        }

        println!("{}", args.join(" "));
        Ok(())
    }
}
