use super::DESTROY;
use clap_builder::{ArgMatches, Command};

pub fn command() -> Command {
    Command::new(DESTROY).about("Remove all items from last deployed location")
}

pub fn handle_matches(matches: &ArgMatches) {
    dbg!(matches);
}
