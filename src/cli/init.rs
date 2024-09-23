use super::INIT;
use clap_builder::{Arg, ArgMatches, Command};
use color_eyre::eyre::Result;
use std::fs;
use std::path;

pub fn command() -> Command {
    Command::new(INIT)
        .about("Initialize the current working directory as a 'dots' directory.")
        .arg(Arg::new("path").required(true))
}

pub fn handle_matches(matches: &ArgMatches) -> Result<()> {
    if let Some(path) = matches.get_one::<String>("path") {
        if path::Path::new(path).exists() {
            initialize_path(path)?;
        } else {
            println!("Please supply an existing path. '.' can be used to initalize the current working directory.");
        }
    }
    Ok(())
}

fn initialize_path(path: &String) -> Result<()> {
    let absolute_path = fs::canonicalize(path::Path::new(path))?;
    dbg!(absolute_path);
    Ok(())
}
