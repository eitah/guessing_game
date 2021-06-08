use std::fs;
use std::error::Error;

pub struct Config {
  pub query: String,
  pub filename: String,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &str> {
    if args.len() < 3 {
      return Err("not enough arguments")
    }

    let query = args[1].clone();
    let filename = args[2].clone();

    Ok(Config{query, filename})
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.filename)?;
  for line in search(&config.query, &contents) {
    println!("{}", line)
  };
  Ok(())
}

// Pseudocode:
  // * iterarte through each contents line
  // * Check whether the line contains our query string
  // * If it does add it to the list of vecs returning
  // * If it doesnt do nothing
  // * Return the list of results that match
// This lifetime tells rust that the life of the vec string is the same as the life
// of the file contents since thats where our pointer will be.
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
  let mut matches: Vec<&str> = vec![];
  for line in contents.lines() {
    if line.contains(query) {
      matches.push(line)
    }
  }

  return matches
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn one_result() {
    let query = "fish";
    let contents = "\
Rust is cool:
It does a lot of things
like cook fish
Greatest Programmer ever.";

    assert_eq!(vec!["like cook fish"],  search(query, contents));
  }
}