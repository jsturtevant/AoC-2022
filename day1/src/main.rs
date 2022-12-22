
use file_import;

fn main() {
    if let Ok(lines) = file_import::getlines("./input.txt") {
        
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
        let first = calorie_totals.pop().unwrap();
        println!("carrying the most {:?}",  first);

        let second =calorie_totals.pop().unwrap();
        let third: i32 = calorie_totals.pop().unwrap();

        let top_three_total = first + second + third;

        println!("total for the top three {:?}",  top_three_total);
    }
}
