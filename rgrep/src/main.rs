use std::{env, process};
use rgrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // No need to unwrap here because Ok returns unit type () and
    // therefore we only care about handling errors
    if let Err(e) = rgrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }

}
