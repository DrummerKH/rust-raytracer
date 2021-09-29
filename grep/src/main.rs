use std::{env, process};
use grep::Config;

fn main() {

    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });

    println!("In file {}", config.filename);

    if let Err(e) = grep::run(config) {
        println!("Error occurred: {}", e);
        process::exit(1);
    }
}


