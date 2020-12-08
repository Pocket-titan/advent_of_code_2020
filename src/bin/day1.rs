use std::fs;
use std::path::Path;

fn main() {
    let path = Path::new("inputs/day1/input.txt");

    let inputs = match fs::read_to_string(path) {
        Err(why) => panic!("Couldn't open {}: {}", path.display(), why),
        Ok(value) => value,
    };

    let lines = inputs.lines();

    for number_one in lines.clone() {
        for number_two in lines.clone() {
            let one = number_one.parse::<i32>().unwrap();
            let two = number_two.parse::<i32>().unwrap();

            // This is kind of hacky - what if the number is 1010 such that itself + itself = 2020?
            if one != two {
                let sum = one + two;

                if sum == 2020 {
                    let result = one * two;
                    println!(
                        "Numbers {} and {} add up to {}, and multiplying them gives: {} * {} = {}!",
                        one, two, sum, one, two, result
                    );
                }
            }
        }
    }
}
