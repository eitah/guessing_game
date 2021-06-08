// The responsibilities that remain in the main function after refactoring
// should be limited to the following:
// *    Calling the command line parsing logic with the argument values
// *    Setting up any other configuration
// *   Calling a run function in lib.rs
// *   Handling the error if run returns an error

use std::env;
use std::fs;

fn main() {
  let args: Vec<String> = env::args().collect();
  let config = parse_config(&args);

  println!("Searching for {}", config.query);
  println!("in file {}", config.filename);

  let contents = fs::read_to_string(config.filename).expect("Something went wrong when reading the file");

  println!("With text:\n{}", contents)
}

struct Config {
  query: String,
  filename: String,
}

fn parse_config(args: &[String]) -> Config {
  let query = args[1].clone();
  let filename = args[2].clone();

  Config{query, filename}
}