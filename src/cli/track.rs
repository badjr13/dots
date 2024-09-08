use super::TRACK;
use anyhow::Result;
use clap_builder::{Arg, ArgMatches, Command};

pub fn command() -> Command {
    Command::new(TRACK).about("Track items to be deployed").arg(
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

    if items.is_empty() {
        println!("Items to be tracked must be passed.")
    }

    if items.contains(&&String::from(".")) {
        println!("Track all items");
    } else {
        println!("Track these items: {:?}", items);
    }

    Ok(())
}
