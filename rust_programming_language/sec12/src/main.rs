use std::env;
use std::process;

use sec12;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Get arguments from command line
    let config = sec12::Config::new(&args).unwrap_or_else(|err| {
        eprintln!("got err {:?}", err);
        process::exit(1);
    });


    if let Err(e) = sec12::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }

}

