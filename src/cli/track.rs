use super::TRACK;
use clap_builder::{Arg, ArgMatches, Command};

pub fn command() -> Command {
    Command::new(TRACK)
        .about("Track everything in current working directory")
        .arg(
            Arg::new("items")
                .help("Items to track")
                .num_args(1..)
                .exclusive(true),
        )
}

pub fn handle_matches(matches: &ArgMatches) {
    dbg!(matches);
}
