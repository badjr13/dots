use clap_builder::{Arg, Command};

pub fn command() -> Command {
    Command::new("track")
        .about("Track everything in current working directory")
        .arg(
            Arg::new("items")
                .help("Items to track")
                .num_args(1..)
                .exclusive(true),
        )
}
