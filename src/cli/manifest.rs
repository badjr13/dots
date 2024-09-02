use super::MANIFEST;
use clap_builder::{ArgMatches, Command};

pub fn command() -> Command {
    Command::new(MANIFEST).about("Manage your manifest file")
}

pub fn handle_matches(matches: &ArgMatches) {
    dbg!(matches);
}
