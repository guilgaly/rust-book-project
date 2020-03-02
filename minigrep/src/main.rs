use std::env;
use std::process;

use minigrep::run;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Unable to parse arguments: {}", err);
        process::exit(1);
    });
    println!("Searching for {} in file {}", config.query, config.filename);

    if let Err(err) = run(config) {
        println!("Application error: {}", err);
        process::exit(1);
    }
}
