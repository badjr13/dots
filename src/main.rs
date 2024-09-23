#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

mod cli;
use color_eyre::eyre::Result;

fn main() -> Result<()> {
    cli::execute()?;
    Ok(())
}
