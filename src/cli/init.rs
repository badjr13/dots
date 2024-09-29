use super::INIT;
use clap_builder::{Arg, ArgMatches, Command};
use color_eyre::eyre::Result;
use std::fs::{canonicalize, read_dir, File, OpenOptions};
use std::io::Write;
use std::path::Path;

const MANIFEST_FILE: &str = "manifest.toml";

pub fn command() -> Command {
    Command::new(INIT)
        .about("Initialize the current working directory as a 'dots' directory.")
        .arg(Arg::new("path").required(true))
}

pub fn handle_matches(matches: &ArgMatches) -> Result<()> {
    if let Some(path) = matches.get_one::<String>("path") {
        if Path::new(path).exists() {
            let absolute_path = canonicalize(path)?;
            let manifest_file = create_manifest(&absolute_path)?;
            track_initial_items(&absolute_path, &manifest_file)?;
        } else {
            println!("Please supply an existing path. '.' can be used to initalize the current working directory.");
        }
    }

    Ok(())
}

fn create_manifest(path: &Path) -> Result<File> {
    let full_path = path.join(MANIFEST_FILE);

    let manifest_file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(full_path)?;

    Ok(manifest_file)
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
