use super::INIT;
use clap_builder::{Arg, ArgMatches, Command};
use color_eyre::eyre::Result;
use std::fs::{canonicalize, read_dir, File, OpenOptions};
use std::io::Write;
use std::path::Path;

const MANIFEST_FILE: &str = ".dots-manifest.toml";

pub fn command() -> Command {
    Command::new(INIT)
        .about("Initialize the current working directory as a 'dots' directory.")
        .arg(Arg::new("path").required(true))
}

pub fn handle_matches(matches: &ArgMatches) -> Result<()> {
    if let Some(path_to_track) = matches.get_one::<String>("path") {
        handle_match_path(path_to_track)?;
    }

    Ok(())
}

fn handle_match_path(path_to_track: &String) -> Result<()> {
    if Path::new(path_to_track).exists() {
        let absolute_path_to_track = canonicalize(path_to_track)?;
        if let Some(manifest_file) = create_manifest(&absolute_path_to_track) {
            track_initial_items(&absolute_path_to_track, &manifest_file)?;
        }
    } else {
        println!("Please supply an existing path. '.' can be used to initalize the current working directory.");
    }

    Ok(())
}

fn create_manifest(path: &Path) -> Option<File> {
    let full_path = path.join(MANIFEST_FILE);

    if full_path.exists() {
        println!("Checking for new items to be tracked.");
        return None;
    }

    OpenOptions::new()
        .append(true)
        .create(true)
        .open(full_path)
        .ok()
}

fn track_initial_items(path: &Path, mut file: &File) -> Result<()> {
    let items = read_dir(path)?;

    for item in items {
        let item = item?.path();
        let item = item.as_path().to_str().unwrap_or_default();
        writeln!(file, "{:?}", &item)?;
    }

    Ok(())
}
