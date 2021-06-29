use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::collections::HashMap;

/* IDEA: Using a hashmap here might be overkill. After all you only need to keep track of one letter
and its corresponding value 

NEXT STEPS: implement a buffer that holds a character and then waits for the next char. If the next
char is not a '\n', add two chars together and convert to int. Otherwise directly convert to int

*/

struct CharStream {
  buffer: [char; 3],
  full: bool,
}

fn create_charstream(buffer: [char; 3], full: bool) -> CharStream {
  CharStream {
    buffer: buffer,
    full: full,
  }
}

fn parse(password: &str) {
  //let mut letters = HashMap::new();
  let mut min = '0';
  // let mut max = '0';
  // let mut ch = '0';
  let buffer: [char; 3] = ['0'; 3];
  let full = false;
  let mut cs = create_charstream(buffer, full);
  for (i, c) in password.chars().enumerate() {
    // min, max and char are always at the same position in input.txt
    if i == 0 {
      cs.buffer[0] = c;
      //min = c;
      println!("First char: {}", cs.buffer[0]);
    } 
    // take care of double digit number for min
    if i == 1 && c != '-' {
      cs.buffer[1] = c;
      println!("second char: {}", cs.buffer[1]);
    }
    
    // if i == 2 {
    //   max = c;
    // } else if i == 4 {
    //   ch = c;
    // } else if i > 6 {
    //   println!("{}", c);
    // }
  }

  println!("min: {}", min);  
  // println!("max: {}", max);
  // println!("char to be tested: {}", ch);
}

fn main() {
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
  parse("11-18 s: ksssssssgssssssssk");
  /*for password in &passwords {
    //println!("password: {}", password);
    //parse(&password);
  }*/
}
