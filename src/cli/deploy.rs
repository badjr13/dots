use super::DEPLOY;
use clap_builder::{ArgMatches, Command};
use color_eyre::eyre::Result;

pub fn command() -> Command {
    Command::new(DEPLOY).about("Deploy tracked items to locations defined in manifest")
}

pub fn handle_matches(matches: &ArgMatches) -> Result<()> {
    dbg!(matches);
    Ok(())
}
