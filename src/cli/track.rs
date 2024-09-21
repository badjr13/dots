use super::TRACK;
use clap_builder::{Arg, ArgMatches, Command};
use color_eyre::eyre::Result;
use std::fs::{canonicalize, read_dir};

pub fn command() -> Command {
    Command::new(TRACK).about("Track items to be deployed").arg(
        Arg::new("items")
            .help("Items to track")
            .num_args(1..)
            .exclusive(true),
    )
}

pub fn handle_matches(matches: &ArgMatches) -> Result<()> {
    // items represent a directory or file to be tracked
    let items: Vec<_> = matches
        .get_many::<String>("items")
        .map(|x| x.cloned().collect())
        .unwrap_or_default();

    if items.is_empty() {
        println!("Pass in items to track or pass in '.' to track all items in the current working directory.");
    }

    if items.len() == 1 && items[0] == "." {
        for entry in read_dir(".")? {
            let absolute_path = canonicalize(entry?.path())?;
            println!("{absolute_path:?}");
        }
    }

    if items.len() > 1 && items.contains(&String::from(".")) {
        println!("A '.' may only be passed by itself.");
    }

    if !items.is_empty() && !items.contains(&String::from(".")) {
        println!("Track these items: {items:?}");
    }

    Ok(())
}
