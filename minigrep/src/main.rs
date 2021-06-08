use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1)
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("App Error: {}", e);
        process::exit(1);
    };
}

// The responsibilities that remain in the main function after refactoring
// are and should be limited to the following:
// *    Calling the command line parsing logic with the argument values
// *    Setting up any other configuration
// *   Calling a run function in lib.rs
// *   Handling the error if run returns an error
