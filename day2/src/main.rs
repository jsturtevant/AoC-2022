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
                let hint =hand_itr.next().unwrap();

                let hand2 = choose(hand1, hint);

                score += play(hand1, hand2);

                match hand2 {
                    ROCK => score += 1,
                    PAPER => score += 2,
                    SCISSOR => score += 3,
                    _ => {}                
                }
            }
        }

        println!("Total: {}", score);
    }
}

const ROCK: &str = "A";
const PAPER: &str = "B";
const SCISSOR: &str = "C";

const RESPONSE_LOOSE: &str = "X";
const RESPONSE_DRAW: &str = "Y";
const RESPONSE_WIN: &str = "Z";

fn choose<'a>(hand1: &'a str, hand2: &'a str) -> &'a str {
    match hand1 {
        ROCK => {   
            match hand2 {
                RESPONSE_LOOSE => SCISSOR,
                RESPONSE_DRAW => ROCK,
                RESPONSE_WIN => PAPER, 
                _ => "",                
            }
        },
        PAPER => {
            match hand2 {
                RESPONSE_LOOSE => ROCK,
                RESPONSE_DRAW => PAPER,
                RESPONSE_WIN => SCISSOR, 
                _ => "",                
            }
        },
        SCISSOR => {
            match hand2 {
                RESPONSE_LOOSE => PAPER,
                RESPONSE_DRAW => SCISSOR,
                RESPONSE_WIN => ROCK, 
                _ => "", 
            }
        },
        _ => "",
    }
}

fn play(hand1: &str, hand2: &str) -> i32 {
    match hand1 {
        ROCK => {   
            match hand2 {
                PAPER =>  6,
                ROCK =>  3,
                _ => 0,                
            }
        },
        PAPER => {
            match hand2 {
                SCISSOR =>  6,
                PAPER => 3,
                _ => 0,                
            }
        },
        SCISSOR => {
            match hand2 {
                ROCK =>  6,
                SCISSOR =>  3,
                _ => 0, 
            }
        },
        _ => 0,
    }
}

// https://doc.rust-lang.org/stable/rust-by-example/std_misc/arg/matching.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    use crate::{ROCK, RESPONSE_DRAW};

    #[test]
    fn choose_should_draw_rock() {
       let h1 = ROCK;
       let hint = RESPONSE_DRAW;

       let h2 = choose(h1, hint);

       assert_eq!(ROCK, h2)
    }

    #[test]
    fn choose_should_win_rock() {
       let h1 = ROCK;
       let hint = RESPONSE_WIN;

       let h2 = choose(h1, hint);

       assert_eq!(PAPER, h2)
    }

    #[test]
    fn choose_should_loose_rock() {
       let h1 = ROCK;
       let hint = RESPONSE_LOOSE;

       let h2 = choose(h1, hint);

       assert_eq!(SCISSOR, h2)
    }

    #[test]
    fn choose_should_draw_paper() {
        let h1 = PAPER;
        let hint = RESPONSE_DRAW;
 
        let h2 = choose(h1, hint);
 
        assert_eq!(PAPER, h2)
     }
 
     #[test]
     fn choose_should_win_paper() {
        let h1 = PAPER;
        let hint = RESPONSE_WIN;
 
        let h2 = choose(h1, hint);
 
        assert_eq!(SCISSOR, h2)
     }
 
     #[test]
     fn choose_should_loose_paper() {
        let h1 = PAPER;
        let hint = RESPONSE_LOOSE;
 
        let h2 = choose(h1, hint);
 
        assert_eq!(ROCK, h2)
     }

     #[test]
     fn choose_should_draw_scissors() {
        let h1 = SCISSOR;
        let hint = RESPONSE_DRAW;
 
        let h2 = choose(h1, hint);
 
        assert_eq!(SCISSOR, h2)
     }
 
     #[test]
     fn choose_should_win_scissors() {
        let h1 = SCISSOR;
        let hint = RESPONSE_WIN;
 
        let h2 = choose(h1, hint);
 
        assert_eq!(ROCK, h2)
     }
 
     #[test]
     fn choose_should_loose_scissors() {
        let h1 = SCISSOR;
        let hint = RESPONSE_LOOSE;
 
        let h2 = choose(h1, hint);
 
        assert_eq!(PAPER, h2)
     }

}