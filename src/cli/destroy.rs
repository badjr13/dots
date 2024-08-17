use clap_builder::Command;

pub fn command() -> Command {
    Command::new("destroy").about("Remove all deployed items from last deployed location")
}
