use super::INIT;
use clap_builder::Command;

pub fn command() -> Command {
    Command::new(INIT).about("Initialize the current working directory as your 'dots' home base.")
}

pub fn handle_matches() {
    dbg!(INIT);
}
