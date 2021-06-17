use std::io;
use std::vec;

const SIZE: i32 = 1000;

fn main() {
  let mut numbers: Vec<i32> = (1..SIZE).collect();
  let mut sum: i32 = 0;
  let mut sum15: i32 = 0;
  for number in numbers.iter() {
    if number % 3 == 0 { 
        println!("Multiple of 3 found at number {}!", number);
        sum += number;
    } else if number % 5 == 0 {
        println!("Multiple of 5 found at number {}!", number);
        sum += number;
    } else if number % 15 == 0 {
        println!("Multiple of 155 found at number {}!", number);
        sum15 += number;
    }
  }
  println!("Total sum: {}", sum);
}