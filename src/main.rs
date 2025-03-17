use std::io::Result;

use kernel::Shell;

mod kernel;

fn main() -> Result<()> {
    Shell::launch()
}
