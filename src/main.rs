#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

use color_eyre::eyre::Result;
mod cli;

fn main() -> Result<()> {
    cli::execute()?;
    Ok(())
}
