use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
/*
A, X - rock = 1
B, Y - paper = 2
C, Z - scissors = 3

lose = 0
draw = 3
win = 6
*/

fn main() {
    let mut total_score = 0;
    // using a 'combo string' approach is pretty brittle but Just enough for this problem
    let winning: [String; 3] = ["A Y".to_string(), "B Z".to_string(), "C X".to_string()];
    let draw: [String; 3] = ["A X".to_string(), "B Y".to_string(), "C Z".to_string()];
    if let Ok(lines) = read_lines("./input/input_2.txt") {
        for line in lines {
            if let Ok(ip) = line {
              let mut score = 0;
              let my_pick = ip.chars().nth(2).unwrap();
              if my_pick == 'X'{
                score += 1;
              } else if my_pick == 'Y'{
                score += 2;
              } else if my_pick == 'Z' {
                score += 3;
              }
              
              if winning.contains(&ip){
                score += 6
              } else if draw.contains(&ip){
                score += 3
              }

              total_score += score;

            }
        }
        println!("total score: {}", total_score);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}