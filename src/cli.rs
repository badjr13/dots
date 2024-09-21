pub mod deploy;
pub mod destroy;
pub mod init;

use clap_builder::{ArgMatches, Command};

const INIT: &str = "init";
const DEPLOY: &str = "deploy";
const DESTROY: &str = "destroy";

pub fn execute() {
    let matches = Command::new("dots")
        .author("badjr13")
        .version(env!("CARGO_PKG_VERSION"))
        .about("A dead simple dotfile manager")
        .subcommand(init::command())
        .subcommand(deploy::command())
        .subcommand(destroy::command())
        .get_matches();

    handle_matches(&matches);
}

fn handle_matches(matches: &ArgMatches) {
    if matches.subcommand_matches(INIT).is_some() {
        init::handle_matches();
    }

    if matches.subcommand_matches(DEPLOY).is_some() {
        deploy::handle_matches();
    }

    if matches.subcommand_matches(DESTROY).is_some() {
        destroy::handle_matches();
    }
}
