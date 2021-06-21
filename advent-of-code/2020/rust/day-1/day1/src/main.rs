use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

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

    // This gives us access to a collection of iterators over each number
    let lines = s.lines();
    let mut numbers: Vec<i32> = Vec::new();

    for line in lines {
        let line = line.parse::<i32>().unwrap();
        numbers.push(line);
    }

    /* Solution for Part 1 */
    for &number in &numbers {
        for &line in &numbers {
            if number + line == 2020 {
                println!("Result for 2 vars: {}", line*number)
            }
        }
    }

    /* Solution for Part 2 */
    for &it1 in &numbers {
        for &it2 in &numbers {
            for &it3 in &numbers {
                if it1 + it2 + it3 == 2020 {
                    println!("Result for 3 vars: {}", it1*it2*it3);
                }
            }
        }
    }
}
