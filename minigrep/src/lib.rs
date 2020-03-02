use std::error::Error;
use std::fs;

pub struct Config<'a> {
    pub query: &'a str,
    pub filename: &'a str,
}

impl Config<'_> {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            Err("not enough arguments")
        } else {
            let query = &args[1];
            let filename = &args[2];
            Ok(Config { query, filename })
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_contents = fs::read_to_string(config.filename)?;
    println!("File text:\n{}", file_contents);
    Ok(())
}
