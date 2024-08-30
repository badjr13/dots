mod cli;

fn main() {
    let x = cli::root().get_matches();
    dbg!(x);
}
