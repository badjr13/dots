use clap_builder::Command;

pub fn command() -> Command {
    Command::new("manifest").about("Manage your manifest file")
}
