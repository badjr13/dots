pub mod deploy;
pub mod destroy;
pub mod init;
use color_eyre::eyre::Result;

use clap_builder::{ArgMatches, Command};

const INIT: &str = "init";
const DEPLOY: &str = "deploy";
const DESTROY: &str = "destroy";

pub fn execute() -> Result<()> {
    let matches = Command::new("dots")
        .author("badjr13")
        .version(env!("CARGO_PKG_VERSION"))
        .about("A dead simple dotfile manager")
        .subcommand(init::command())
        .subcommand(deploy::command())
        .subcommand(destroy::command())
        .get_matches();

    handle_matches(&matches)?;

    Ok(())
}

fn handle_matches(matches: &ArgMatches) -> Result<()> {
    if let Some(init_matches) = matches.subcommand_matches(INIT) {
        init::handle_matches(init_matches)?;
    }

    if let Some(deploy_matches) = matches.subcommand_matches(DEPLOY) {
        deploy::handle_matches(deploy_matches);
    }

    if let Some(destroy_matches) = matches.subcommand_matches(DESTROY) {
        destroy::handle_matches(destroy_matches);
    }

    Ok(())
}
