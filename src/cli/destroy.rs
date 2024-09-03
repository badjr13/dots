use super::DESTROY;
use clap_builder::Command;

pub fn command() -> Command {
    Command::new(DESTROY).about("Remove all deployed items from last deployed location")
}

pub fn handle_matches() {
    destroy();
}

fn destroy() {
    println!("Destroying dot files");
}
