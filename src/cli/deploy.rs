use super::DEPLOY;
use clap_builder::Command;

pub fn command() -> Command {
    Command::new(DEPLOY).about("Deploy tracked items to locations defined in manifest")
}

// pub fn handle_matches(matches: &ArgMatches) -> Result<()> {
//     // items represent a directory or file to be tracked
//     let items: Vec<_> = matches
//         .get_many::<String>("items")
//         .map(|x| x.cloned().collect())
//         .unwrap_or_default();

//     if items.is_empty() {
//         println!("Pass in items to track or pass in '.' to track all items in the current working directory.");
//     }

//     if items.len() == 1 && items[0] == "." {
//         for entry in read_dir(".")? {
//             let absolute_path = canonicalize(entry?.path())?;
//             println!("{absolute_path:?}");
//         }
//     }

//     if items.len() > 1 && items.contains(&String::from(".")) {
//         println!("A '.' may only be passed by itself.");
//     }

//     if !items.is_empty() && !items.contains(&String::from(".")) {
//         println!("Track these items: {items:?}");
//     }

//     Ok(())
// }

pub fn handle_matches() {
    deploy();
}

fn deploy() {
    println!("Deploying dot files");
}
