use {
    super::CommandHandler, crate::rgb_to_ansi256, std::{fs::File, io::{self, BufRead}}
};

pub struct History;

impl CommandHandler for History {
    fn execute(args: &[String]) -> Result<(), std::io::Error> {
        if args.is_empty() {
            let file = File::open("logs/history.log")?;
            let reader = io::BufReader::new(file);

            let lines: Vec<_> = reader.lines().collect::<Result<_, _>>()?;
            let start = if lines.len() > 16 { lines.len() - 16 } else { 0 };

            for (i, line) in lines.iter().skip(start).enumerate() {
                println!("{:>5}  {}", start + i + 1, line);
            }
            return Ok(());
        }

        Err(io::Error::new(
            io::ErrorKind::InvalidInput, 
            format!("\x1b[1;3mhistory:\x1b[0m \x1b[38;5;{}mtoo many arguments\x1b[0m", rgb_to_ansi256(220, 45, 34))
        ))
    }
}
