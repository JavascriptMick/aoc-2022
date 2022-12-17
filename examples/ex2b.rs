use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{HashMap};
/*
Their choice
A - rock
B - paper
C - scissors

if you choose..
rock = 1
paper = 2
scissors = 3

X - lose = 0
Y - draw = 3
Z - win = 6
*/

fn main() {
    let mut total_score = 0;

    #[derive(Debug, PartialEq, Eq, Hash)]
    enum Hand {
      ROCK,
      PAPER,
      SCISSORS
  }
    let their_choice_map = HashMap::from([('A', Hand::ROCK), ('B', Hand::PAPER), ('C', Hand::SCISSORS)]);
    let hand_score = HashMap::from([(Hand::ROCK, 1), (Hand::PAPER, 2), (Hand::SCISSORS, 3)]);
    let to_win = HashMap::from([(Hand::ROCK, Hand::PAPER), (Hand::PAPER, Hand::SCISSORS), (Hand::SCISSORS, Hand::ROCK)]);
    let to_lose = HashMap::from([(Hand::ROCK, Hand::SCISSORS), (Hand::PAPER, Hand::ROCK), (Hand::SCISSORS, Hand::PAPER)]);

    if let Ok(lines) = read_lines("./input/input_2.txt") {
        for line in lines {
            if let Ok(ip) = line {
              let mut score = 0;

              let their_choice_char = ip.chars().next().unwrap();
              let their_choice = &their_choice_map[&their_choice_char];

              let my_win_lose_char = ip.chars().nth(2).unwrap();
              if my_win_lose_char == 'X'{ // lose
                let my_choice = &to_lose[their_choice];
                score += hand_score[my_choice];
              } else if my_win_lose_char == 'Y'{ // draw
                score += &hand_score[their_choice]; // I choose same as them to draw
                score += 3; // for the draw
              } else if my_win_lose_char == 'Z' { // win
                let my_choice = &to_win[their_choice];
                score += hand_score[my_choice];
                score += 6; // for the win
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