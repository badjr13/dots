use super::DESTROY;
use clap_builder::{ArgMatches, Command};
use color_eyre::eyre::Result;

pub fn command() -> Command {
    Command::new(DESTROY).about("Remove all items from last deployed location")
}

pub fn handle_matches(matches: &ArgMatches) -> Result<()> {
    dbg!(matches);
    Ok(())
}
