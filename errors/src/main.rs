use std::fs;
use std::fs::File;
#[allow(unused_imports)]
use std::io::ErrorKind;
use std::io;
#[allow(unused_imports)]
use std::io::Read;

#[allow(unused_variables)]
fn main() {
  let f = File::open("hello.txt");

  // less robust example for baby steps  reasons
  // let f = match f {
  //   Ok(file) => file,
  //   Err(error) => match error.kind() {
  //     ErrorKind::NotFound => match File::create("hello.txt") {
  //       Ok(fc) => fc,
  //       Err(e) => panic!("Problem creating the file: {:?}", error)
  //     },
  //     other_error => {
  //       panic!("Problem opening the file: {:?}", other_error)
  //     }
  //   }
  // };

    // more concise  example for when we learn about this
  // let f = File::open("hello.txt").unwrap_or_else(|error| {
  //   if error.kind() == ErrorKind::NotFound {
  //     File::create("hello.txt").unwrap_or_else(|error| {
  //       panic!("Problem creating the file: {:?}", error)
  //     })
  //   } else {
  //     panic!("Problem opening the file: {:?}", error)
  //   }
  // });

  // unwrap gives us the error if its an error or the result if its a success,
  // let f = File::open("hello.txt").unwrap();

  // expect gives a clearer error message than unwrap but does the same thing
  // let f = File::open("hello.txt").expect("Failed to open hello.txt");

  let f = read_username_from_file();
  println!("result of read username call is {:?}", f)
}

// a semi-simple implementation of read username from file
// fn read_username_from_file() -> Result<String, io::Error> {
//   let mut f = File::open("hello.txt")?;
//   let mut s = String::new();
//   // ? means if result is ok it will be returned else it will error.
//   f.read_to_string(&mut s)?;
//   Ok(s)
// }

// this implementation is just showing off lol
// fn read_username_from_file() -> Result<String, io::Error> {
//   fs::read_to_string("hello.txt")
// }

// real-world-ish example?
fn read_username_from_file() -> Result<String, io::Error> {
  let mut s = String::new();

  File::open("hello.txt")?.read_to_string(&mut s)?;

  Ok(s)
}
