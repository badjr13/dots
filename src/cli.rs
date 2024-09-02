pub mod deploy;
pub mod destroy;
pub mod manifest;
pub mod track;

use clap_builder::{ArgMatches, Command};

const TRACK: &str = "track";
const DEPLOY: &str = "deploy";
const DESTROY: &str = "destroy";
const MANIFEST: &str = "manifest";

pub fn execute() {
    let matches = Command::new("dots")
        .author("badjr13")
        .version(env!("CARGO_PKG_VERSION"))
        .about("A dead simple dotfile manager")
        .subcommand(track::command())
        .subcommand(deploy::command())
        .subcommand(destroy::command())
        .subcommand(manifest::command())
        .get_matches();

    handle_matches(&matches);
}

fn handle_matches(matches: &ArgMatches) {
    if let Some(track_matches) = matches.subcommand_matches(TRACK) {
        track::handle_matches(track_matches);
    }

    if let Some(deploy_matches) = matches.subcommand_matches(DEPLOY) {
        deploy::handle_matches(deploy_matches)
    }

    if let Some(destroy_matches) = matches.subcommand_matches(DESTROY) {
        destroy::handle_matches(destroy_matches)
    }

    if let Some(manifest_matches) = matches.subcommand_matches("manifest") {
        manifest::handle_matches(manifest_matches)
    }
}
