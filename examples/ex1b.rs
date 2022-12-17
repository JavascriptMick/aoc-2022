use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut current_agg = 0;
    let mut max_aggs : Vec<i32> = Vec::new();
    let mut aggs_index = 0;
    if let Ok(lines) = read_lines("./input/input_1.txt") {
        for line in lines {
            if let Ok(ip) = line {
              if ip.len() == 0 {
                max_aggs.push(current_agg);
                current_agg = 0;
              } else {
                let my_int = ip.parse::<i32>().unwrap();
                current_agg += my_int;
              }
            }
        }
        max_aggs.sort_by(|a, b| b.cmp(a));
        println!("Final aggs {}, {}, {}", max_aggs[0], max_aggs[1], max_aggs[2]);
        let total = max_aggs[0] + max_aggs[1] + max_aggs[2];
        println!("total of top 3: {}", total);

    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}