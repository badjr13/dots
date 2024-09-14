use color_eyre::eyre::Result;
mod cli;

fn main() -> Result<()> {
    cli::execute()?;
    Ok(())
}
