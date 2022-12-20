use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        
        let mut score = 0;
        for line in lines {
            if let Ok(hands) = line {
                let mut hand_itr = hands.split_whitespace();
                let hand1 = hand_itr.next().unwrap();
                let hand2 =hand_itr.next().unwrap();

                score += play(hand1, hand2)
            }
        }


        println!("Total: {}", score);
    }
}

const ROCK: &str = "A";
const PAPER: &str = "B";
const SCISSOR: &str = "C";

const RESPONSE_ROCK: &str = "X";
const RESPONSE_PAPER: &str = "Y";
const RESPONSE_SCISSOR: &str = "Z";


fn play(hand1: &str, hand2: &str) -> i32 {
    let mut score = 0;
    match hand1 {
        ROCK => {   
            match hand2 {
                RESPONSE_PAPER => score = 6,
                RESPONSE_ROCK => score = 3,
                _ => {},                
            }
        },
        PAPER => {
            match hand2 {
                RESPONSE_SCISSOR => score = 6,
                RESPONSE_PAPER => score = 3,
                _ => {},                
            }
        },
        SCISSOR => {
            match hand2 {
                RESPONSE_ROCK => score = 6,
                RESPONSE_SCISSOR => score = 3,
                _ => {}, 
            }
        },
        _ => println!("unknown"),
    }

    match hand2 {
        RESPONSE_PAPER => score += 2,
        RESPONSE_ROCK => score += 1,
        RESPONSE_SCISSOR => score += 3,
        _ => {}                
    }

    score
}

// https://doc.rust-lang.org/stable/rust-by-example/std_misc/arg/matching.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}