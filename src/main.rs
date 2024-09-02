mod cli;

fn main() {
    let matches = cli::root().get_matches();

    if let Some(track_match) = matches.subcommand_matches("track") {
        dbg!(track_match);
    }

    if let Some(deploy_match) = matches.subcommand_matches("deploy") {
        dbg!(deploy_match);
    }

    if let Some(destroy_match) = matches.subcommand_matches("destroy") {
        dbg!(destroy_match);
    }

    if let Some(manifest_match) = matches.subcommand_matches("manifest") {
        dbg!(manifest_match);
    }
}
