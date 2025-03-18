use super::CommandHandler;

pub struct ChangeDirectory;

impl CommandHandler for ChangeDirectory {
    fn execute(args: &[String]) -> Result<(), std::io::Error> {
        Ok(())
    }
}
