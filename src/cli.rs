pub mod deploy;
pub mod destroy;
pub mod track;

use clap_builder::Command;

pub fn root() -> Command {
    Command::new("dots")
        .author("badjr13")
        .version("0.1.0")
        .about("A dead simple dotfile manager")
        .subcommand(track::command())
        .subcommand(deploy::command())
        .subcommand(destroy::command())
}
