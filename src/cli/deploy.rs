use super::DEPLOY;
use clap_builder::Command;

pub fn command() -> Command {
    Command::new(DEPLOY).about("Deploy tracked items to locations defined in manifest")
}

pub fn handle_matches() {
    deploy();
}

fn deploy() {
    println!("Deploying dot files");
}
