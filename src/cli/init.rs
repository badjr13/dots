use super::INIT;
use clap_builder::{Arg, ArgMatches, Command};
use color_eyre::eyre::Result;
use std::fs;
use std::path::Path;

pub fn command() -> Command {
    Command::new(INIT)
        .about("Initialize the current working directory as a 'dots' directory.")
        .arg(Arg::new("path").required(true))
}

pub fn handle_matches(matches: &ArgMatches) -> Result<()> {
    if let Some(path) = matches.get_one::<String>("path") {
        if Path::new(path).exists() {
            let absolute_path = fs::canonicalize(path)?;
            create_manifest(&absolute_path)?;
            track_initial_items(&absolute_path)?;
        } else {
            println!("Please supply an existing path. '.' can be used to initalize the current working directory.");
        }
    }
    Ok(())
}

fn create_manifest(path: &Path) -> Result<()> {
    let manifest_path = path.join("manifest.toml");
    fs::File::create(&manifest_path)?;
    Ok(())
}

fn track_initial_items(path: &Path) -> Result<()> {
    let items = fs::read_dir(path)?;
    for item in items {
        dbg!(item?);
    }
    Ok(())
}
