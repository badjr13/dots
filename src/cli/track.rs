use super::TRACK;
use anyhow::Result;
use clap_builder::{Arg, ArgMatches, Command};

const ITEMS: &str = "items";

pub fn command() -> Command {
    Command::new(TRACK)
        .about("Track everything in current working directory")
        .arg(
            Arg::new(ITEMS)
                .help("Items to track")
                .num_args(1..)
                .exclusive(true),
        )
}

pub fn handle_matches(matches: &ArgMatches) -> Result<()> {
    let x: Vec<_> = matches
        .get_many::<String>(ITEMS)
        .unwrap_or_default()
        .collect();
    dbg!(x);
    Ok(())
}
