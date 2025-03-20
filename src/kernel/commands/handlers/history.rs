use {
    super::CommandHandler,
    crate::Error,
    std::{
        fs::File,
        io::{self, BufRead}
    }
};

pub struct History;

impl CommandHandler for History {
    fn execute(args: &[String]) -> Result<(), std::io::Error> {
        if args.is_empty() {
            let file = File::open("logs/history.log")?;
            let reader = io::BufReader::new(file);

            let lines: Vec<_> = reader.lines().collect::<Result<_, _>>()?;
            let mut line_number = 1;
            let mut unique_lines = Vec::new();

            for line in lines {
                if let Some(pos) = unique_lines.iter().position(|unique_line: &Line| unique_line.content == line) {
                    unique_lines.remove(pos);
                }
                unique_lines.push(Line {number: line_number, content: line});
                line_number += 1;
            }

            let start: usize = if unique_lines.len() > 16 { unique_lines.len() - 16 } else { 0 };
            for line in unique_lines.iter().skip(start) {
                println!("{:>5}  {}", line.number, line.content);
            }
            return Ok(());
        }

        Err(Error::throw(io::ErrorKind::ArgumentListTooLong, &["history:"]))
    }
}

struct Line {
    number:  usize,
    content: String
}
