use super::TRACK;
use anyhow::Result;
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

pub fn handle_matches(matches: &ArgMatches) -> Result<()> {
    let items: Vec<_> = matches
        .get_many::<String>("items")
        .unwrap_or_default()
        .collect();

    println!("Items to track: {:#?}", items);

    Ok(())
}
