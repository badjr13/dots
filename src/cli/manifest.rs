use super::MANIFEST;
use anyhow::Result;
use clap_builder::{Arg, ArgAction, ArgMatches, Command};

pub fn command() -> Command {
    Command::new(MANIFEST)
        .about("View and edit manifest file")
        .arg(
            Arg::new("show")
                .help("Show manifest file")
                .short('s')
                .long("show")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("edit")
                .help("Edit manifest file")
                .short('e')
                .long("edit")
                .action(ArgAction::SetTrue),
        )
}

pub fn handle_matches(matches: &ArgMatches) -> Result<()> {
    if let Some(show) = matches.get_one::<bool>("show") {
        println!("Show manifest: {show}")
    }

    if let Some(edit) = matches.get_one::<bool>("edit") {
        println!("Edit manifest: {edit}")
    }

    Ok(())
}
