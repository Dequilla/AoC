pub mod day_1;
pub mod day_2;
pub mod day_3;
pub mod aoc_utility;

fn main() {
    let mut results : Vec<(String, (bool, String))> = Vec::new();

    results.push( ( String::from("Day 1 part 1"), day_1::day_1::run_part1() ) );
    results.push( ( String::from("Day 1 part 2"), day_1::day_1::run_part2() ) );
    results.push( ( String::from("Day 2 part 1"), day_2::day_2::run_part1() ) );
    results.push( ( String::from("Day 2 part 2"), day_2::day_2::run_part2() ) );
    results.push( ( String::from("Day 3 part 1"), day_3::day_3::run_part1() ) );

    for result in results {
        println!("{} -> {} # {}", result.0, result.1.0, result.1.1);
    }
}
