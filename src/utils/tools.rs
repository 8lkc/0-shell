use std::{
    fs::{File, OpenOptions},
    io::{self, BufReader, Write},
    path::Path
};

pub struct Tool;

impl Tool {
    pub fn boldify(string: &str) -> String {
        format!("\x1b[1m{}\x1b[0m", string)
    }

    pub fn colorize(string: &str, color: (u8, u8, u8)) -> String {
        format!("\x1b[38;5;{}m{}\x1b[0m", Self::rgb_to_ansi256(color), string)
    }

    pub fn italicify(string: &str) -> String {
        format!("\x1b[3m{}\x1b[0m", string)
    }

    pub fn push_to_history(line: &str) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("logs/history.log")?;
        writeln!(file, "{}", line)?;
        Ok(())
    }

    pub fn read_file<P: AsRef<Path>>(path: P) -> Result<BufReader<File>, io::Error> {
        let file = File::open(path)?;
        Ok(io::BufReader::new(file))
    }

    fn rgb_to_ansi256(rgb: (u8, u8, u8)) -> u8 {
        let r = (rgb.0 as f32 / 255.0 * 5.0).round() as u8;
        let g = (rgb.1 as f32 / 255.0 * 5.0).round() as u8;
        let b = (rgb.2 as f32 / 255.0 * 5.0).round() as u8;
        16 + 36 * r + 6 * g + b
    }
}
