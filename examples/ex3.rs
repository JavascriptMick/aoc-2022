use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/*
Priority
Lowercase item types a through z have priorities 1 through 26.
Uppercase item types A through Z have priorities 27 through 52.
// A = 65
// a = 97
*/

fn main() {
    let mut total_priority: u32 = 0;

    if let Ok(lines) = read_lines("./input/input_3.txt") {
        for line in lines {
            if let Ok(ip) = line {
              let half_len = ip.len()/2;
              let first_bit = &ip[0..half_len]; // slice slice baby
              let second_bit = &ip[half_len..];
              let mut found: char = 'a'; // brittle, will miscalculate if there is no common char
              for element in first_bit.chars() {
                if second_bit.contains(element) {
                  found = element;
                }
              }
              let found_num = found as u32;
              let priority;
              if found_num > 90 { //lowercase
                priority = found_num - 'a' as u32 + 1;
              } else { // uppercase
                priority = found_num - 'A' as u32 + 27;
              }
              total_priority += priority;
            }
        }
        println!("total priority: {}", total_priority);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}