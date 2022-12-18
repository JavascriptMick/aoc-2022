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
    // let mut group_ip : Vec<String> = Vec::new();  //could probably be an array of len 3
    let mut group_ip: [String; 3] = ["".to_string(), "".to_string(), "".to_string()];
    let mut group_index = 0;

    if let Ok(lines) = read_lines("./input/input_3.txt") {
        for line in lines {
            if let Ok(ip) = line {
              group_ip[group_index] = ip;
              group_index += 1;
              if group_index > 2 {
                group_index = 0;
                let mut found: char = 'a';
                for element in group_ip[0].chars() {
                  if group_ip[1].contains(element) && group_ip[2].contains(element) {
                    found = element;
                  }
                }
                // println!("1: {}, 2: {}, 3: {}, found: {}", group_ip[0], group_ip[1], group_ip[2], found);
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