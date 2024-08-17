use clap_builder::Command;

pub fn command() -> Command {
    Command::new("deploy").about("Deploy tracked items to locations defined in manifest")
}
