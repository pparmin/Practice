use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::collections::HashMap;

//impl FromIterator<char> for String

fn parse(password: &str) {
  let mut letters = HashMap::new();
  let mut min = password.as_bytes()[0];
  let mut max = password.as_bytes()[2];

  println!("min: {}, max: {}", min, max);
  for c in password.chars() {

  }
  letters.insert('c', 0);
  check_validity();
}

fn check_validity() -> bool {
  

  return false;
}

fn main() {
  let valid: i32 = 0;

  let path = Path::new("input.txt");
  let display = path.display();
  println!("{}", display);

  let file = match File::open(&path) {
      Err(why) => panic!("couldn't open {}: {}", display, why),
      Ok(file) => file,
  };

  let mut buffer = BufReader::new(file);
  let mut s = String::new();
  match buffer.read_to_string(&mut s) {
      Err(why) => panic!("couldn't read {}: {}", display, why),
      Ok(s) => s,
  };

  /* First step: 1. Collect every password as a string into a Vec of strings --> 2. Give string to parsing function
  --> 3. Go through each string character by character (either .chars or .chars_as_bytes)

  */
  let passwords: Vec<&str> = s.lines().collect();
  for password in &passwords {
    println!("password: {}", &password);
    parse(&password);
  }
}
