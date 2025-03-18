use {
    super::CommandHandler, crate::rgb_to_ansi256, std::{fs::File, io::{self, BufRead}}
};

pub struct History;

impl CommandHandler for History {
    fn execute(args: &[String]) -> Result<(), std::io::Error> {
        if args.is_empty() {
            let mut number: u32 = 1;
            let file = File::open("logs/history.log")?;
            let reader = io::BufReader::new(file);
            for line in reader.lines() {
                println!("{:>5}  {}", number, line?);
                number += 1;
            }
            return Ok(());
        }
        Err(io::Error::new(
            io::ErrorKind::InvalidInput, 
            format!("\x1b[1;3mhistory:\x1b[0m \x1b[38;5;{}mtoo many arguments\x1b[0m", rgb_to_ansi256(220, 45, 34))
        ))
    }
}
