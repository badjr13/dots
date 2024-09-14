use super::TRACK;
use clap_builder::{Arg, ArgMatches, Command};
use color_eyre::eyre::Result;

pub fn command() -> Command {
    Command::new(TRACK).about("Track items to be deployed").arg(
        Arg::new("items")
            .help("Items to track")
            .num_args(1..)
            .exclusive(true),
    )
}

pub fn handle_matches(matches: &ArgMatches) -> Result<()> {
    let items: Vec<_> = matches
        .get_many::<String>("items")
        .map(|x| x.cloned().collect())
        .unwrap_or_default();

    if items.is_empty() {
        println!("Pass in items to track or pass in '.' to track all items in current working directory.");
    } else if items.contains(&String::from(".")) {
        println!("Track all items");
    } else {
        println!("Track these items: {:?}", items);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle_matches() {
        assert_eq!(1, 1);
    }
}
