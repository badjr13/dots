use super::DEPLOY;
use clap_builder::{ArgMatches, Command};

pub fn command() -> Command {
    Command::new(DEPLOY).about("Deploy tracked items to locations defined in manifest")
}

pub fn handle_matches(matches: &ArgMatches) {
    dbg!(matches);
}
