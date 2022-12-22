use std::collections::HashMap;
use file_import;

fn main() {
    if let Ok(lines) = file_import::getlines("./input.txt") {
        
        let mut game = RpsGame::new();

        game.game_rules.insert(
            Rps::Rock,
            Rules {
                loses_against: Rps::Paper,
                wins_against: Rps::Scissors,
            },
        );
        game.game_rules.insert(
            Rps::Paper,
            Rules {
                loses_against: Rps::Scissors,
                wins_against: Rps::Rock,
            },
        );
        game.game_rules.insert(
            Rps::Scissors,
            Rules {
                loses_against: Rps::Rock,
                wins_against:  Rps::Paper,
            },
        );

        let mut score = 0;
        for line in lines {
            if let Ok(hands) = line {
                let mut hand_itr = hands.split_whitespace();

                let hand1 = RpsGame::to_hand(hand_itr.next().unwrap().to_string());
                let hint = RpsGame::to_strategy(hand_itr.next().unwrap().to_string());

                let hand2 = game.choose(hand1, hint);

                score += game.play(hand1, hand2);
                score += RpsGame::additional_score(hand2)
            }
        }

        println!("Total: {}", score);
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
enum Rps {
    Rock,
    Paper,
    Scissors,
}

#[derive(Eq, PartialEq, Hash)]
enum Strategy {
    Win,
    Lose,
    Draw
}
#[derive(Clone, Copy)]
struct Rules {
    wins_against: Rps,
    loses_against: Rps,
}

struct RpsGame {
    game_rules:  HashMap<Rps, Rules>,
}

impl RpsGame {
    fn new() -> RpsGame {
        RpsGame {
            game_rules: HashMap::new(),
        }
    }
    fn choose(&self, hand1: Rps, hint: Strategy) -> Rps {
        let rule = self.game_rules[&hand1];

        match hint {
            Strategy::Lose => rule.wins_against,
            Strategy::Win => rule.loses_against,
            Strategy::Draw  => hand1,
        }
    }

    fn play(&self, hand1: Rps, hand2: Rps) -> i32 {
        let rule = self.game_rules[&hand2];

        if hand1 == hand2 {
            return 3
        }

        if rule.loses_against == hand1 {
                return 0
        }else if rule.wins_against == hand1 {
            return 6
        }

       panic!("don't know how to play hand")
    }

    fn additional_score(hand2: Rps) -> i32{
        match hand2 {
            Rps::Rock => 1,
            Rps::Paper => 2,
            Rps::Scissors => 3,
        }
    }

    fn to_strategy(s: String) -> Strategy{
        match s.as_str() {
            "X" => Strategy::Lose,
            "Y" => Strategy::Draw,
            "Z" => Strategy::Win,
            _ => panic!("unknown Strategy")
        }
    }

    fn to_hand(s: String) -> Rps{
        match s.as_str() {
            "A" => Rps::Rock,
            "B" => Rps::Paper,
            "C" => Rps::Scissors,
            _ => panic!("unknown hand")
        }
    }
}

