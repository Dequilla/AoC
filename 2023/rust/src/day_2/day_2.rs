use crate::aoc_utility::aoc_utility;
use std::collections::HashMap;

#[repr(i64)]
#[derive(Debug, PartialEq, Eq, Hash)]
enum Color {
    Red, 
    Green, 
    Blue
}

impl Color {
    fn parse(input: &str) -> Self {
        return match input {
            "red" => Color::Red,
            "blue" => Color::Blue,
            "green" => Color::Green,
            _ => Color::Red
        };
    }
}

#[derive(Debug)]
struct Cube {
    count: i64,
    color: Color
}

#[derive(Debug)]
struct Round {
    cubes: Vec<Cube>
}

#[derive(Debug)]
struct Game {
    index: i64,
    possible: bool,
    rounds: Vec<Round>
}

impl Game {

    fn new() -> Self {
        return Self {
            index: 0,
            possible: false,
            rounds: Vec::new()
        };
    }

    fn parse(input : &str) -> Vec<Self> {
        let mut games: Vec<Game> = Vec::new();
        let lines : Vec<&str> = input.lines().collect();
        for line in lines {
            
            let line_parts : Vec<&str> = line.split(":").collect();
            let str_game = line_parts.first().unwrap();
            let mut game = Game::new(); 
            let vec_game_data: Vec<&str> = str_game.trim().split(" ").collect();
            game.index = vec_game_data.last().unwrap().parse::<i64>().unwrap(); // TODO

            let str_rounds : Vec<&str> = line_parts.last().unwrap().split(";").collect(); 
            for round in str_rounds {
                let cubes : Vec<Cube> = round.split(",")
                    .map(|cube_str| {
                        let x : Vec<&str> = cube_str.trim().split(" ").collect();
                        return Cube {
                            count: x.first().unwrap().parse::<i64>().unwrap(),
                            color: Color::parse(x.last().unwrap())
                        };
                    })
                    .collect();
                
                game.rounds.push( Round {
                    cubes: cubes
                });

            }

            games.push(game);
        }

        return games;
    }

}

pub fn run_part1() -> (bool, String)
{
    //let input = aoc_utility::get_input("input_example.txt", "day_2");
    let input = aoc_utility::get_input("input.txt", "day_2");
    let mut games = Game::parse(&input);

    let max_cubes: [Cube; 3] = [
        Cube { count: 12, color: Color::Red },
        Cube { count: 13, color: Color::Green},
        Cube { count: 14, color: Color::Blue }
    ];

    for game in &mut games {
        let mut max_found_cubes: [Cube; 3] = [
            Cube { count: 0, color: Color::Red },
            Cube { count: 0, color: Color::Green},
            Cube { count: 0, color: Color::Blue }
        ];

        println!("############### Game({}) ###############", game.index);
        for round in &mut game.rounds {
            println!(".--- Round ---.");
            for cube in &mut round.cubes {
                match cube.color {
                    Color::Red => max_found_cubes[0].count = std::cmp::max(max_found_cubes[0].count, cube.count),
                    Color::Green => max_found_cubes[1].count = std::cmp::max(max_found_cubes[1].count, cube.count),
                    Color::Blue => max_found_cubes[2].count = std::cmp::max(max_found_cubes[2].count, cube.count),
                };

                print!("Cube({:?}, {}), ", cube.color, cube.count)
            }
            println!("");
        }
        
        if
            max_cubes[0].count >= max_found_cubes[0].count &&
            max_cubes[1].count >= max_found_cubes[1].count &&
            max_cubes[2].count >= max_found_cubes[2].count 
        {
            game.possible = true;   
        }

        println!("");
    }

    let sum_of_ids : i64 = games.iter()
        .filter(|game| game.possible )
        .map(|game| game.index )
        .sum();

    return (false, format!("Sum of IDs \"{}\"", sum_of_ids));
}