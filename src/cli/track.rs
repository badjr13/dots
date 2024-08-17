use clap_builder::Command;

pub fn command() -> Command {
    Command::new("track").about("Track everything in current working directory")
}
