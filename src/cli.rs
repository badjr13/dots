pub mod deploy;
pub mod destroy;
pub mod manifest;
pub mod track;

use clap_builder::Command;

pub fn root() -> Command {
    Command::new("dots")
        .author("badjr13")
        .version(env!("CARGO_PKG_VERSION"))
        .about("A dead simple dotfile manager")
        .subcommand(track::command())
        .subcommand(deploy::command())
        .subcommand(destroy::command())
        .subcommand(manifest::command())
}
