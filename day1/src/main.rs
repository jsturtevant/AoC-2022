use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        
        let mut calorie_totals = Vec::new();

        let mut elfcalories: i32 = 0;
        for line in lines {
            
            if let Ok(calories) = line {
                match calories.as_str() {
                    "" => {
                        calorie_totals.push(elfcalories);
                        elfcalories = 0;
                    },
                    _ => elfcalories += calories.parse::<i32>().unwrap(),
                }
            }
        }

        calorie_totals.sort();
        let first = calorie_totals.last().unwrap();
        println!("carrying the most {:?}",  first);

        let len = calorie_totals.len();
        let second = calorie_totals[len - 2];
        let third: i32 = calorie_totals[len -3];

        let top_three_total = first + second + third;

        println!("total for the top three {:?}",  top_three_total);
    }
}

// https://doc.rust-lang.org/stable/rust-by-example/std_misc/arg/matching.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
