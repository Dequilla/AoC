mod day_1;

use std::string::String;

fn main() {
    let mut results : Vec<(String, (bool, String))> = Vec::new();

    results.push( ( String::from("Day 1 part 1"), day_1::day_1::run_part1() ) );
    results.push( ( String::from("Day 1 part 2"), day_1::day_1::run_part2() ) );

    for result in results {
        println!("{} -> {} # {}", result.0, result.1.0, result.1.1);
    }
}
