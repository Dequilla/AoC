use std::fs;
use std::env;

fn get_input(filename : &str) -> String
{
    let mut cwd = env::current_dir().unwrap();
    cwd.push("src\\day_1");
    cwd.push(filename);
    // println!("CWD: \"{}\"", cwd.display());

    let input : String = fs::read_to_string( cwd )
        .expect("\"src\\day_1\\input.txt\" not found.");

    return input;
}

pub fn run_part1() -> (bool, String)
{
    let input = get_input("input.txt");

    let mut total_sum : i64 = 0;

    let lines = input.split("\n");
    for line in lines {
        let res : Vec<_> = line.chars()
            .filter(|c| c.is_numeric())
            .collect();

        if res.len() <= 0 {
            continue;
        }

        let first = res.first().unwrap().to_string();
        let last = res.last().unwrap().to_string();
        let combined = format!("{}{}", first, last);

        //println!("{} -> {} = {}", first, last, combined);
        total_sum += combined.parse::<i64>().unwrap();
    }
    
    return (
        true,
        format!("Day 1 part 1 => Total sum of first and last in each row is: {}", total_sum)
    );
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
enum Digits {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9
}

impl Digits {

    #[allow(dead_code)]
    fn as_string(self) -> &'static str {
        match self {
            Self::One   => "one",
            Self::Two   => "two",
            Self::Three => "three",
            Self::Four  => "four",
            Self::Five  => "five",
            Self::Six   => "six",
            Self::Seven => "seven",
            Self::Eight => "eight",
            Self::Nine  => "nine",
        }
    } 

    fn as_char(self) -> char {
        match self {
            Self::One   => '1',
            Self::Two   => '2',
            Self::Three => '3',
            Self::Four  => '4',
            Self::Five  => '5',
            Self::Six   => '6',
            Self::Seven => '7',
            Self::Eight => '8',
            Self::Nine  => '9',
        }
    }

    fn from_str(input : &str) -> Option<Self> {
        match input {
            "one" => Some(Digits::One),
            "two" => Some(Digits::Two),
            "three" => Some(Digits::Three),
            "four" => Some(Digits::Four),
            "five" => Some(Digits::Five),
            "six" => Some(Digits::Six),
            "seven" => Some(Digits::Seven),
            "eight" => Some(Digits::Eight),
            "nine" => Some(Digits::Nine),
            _ => None
        }
    }

    fn contains(input : &str) -> Option<Self> {
        let digits = [
            "one",
            "two",
            "three",
            "four",
            "five", 
            "six",
            "seven",
            "eight",
            "nine", 
        ];

        for d in digits {
            let res = input.contains(d);
            if res {
                return Digits::from_str(d);
            }
        }

        return None;
    }

    fn from_char(input : char) -> Option<Self> {
        match input {
            '1' => Some(Digits::One),
            '2' => Some(Digits::Two),
            '3' => Some(Digits::Three),
            '4' => Some(Digits::Four),
            '5' => Some(Digits::Five),
            '6' => Some(Digits::Six),
            '7' => Some(Digits::Seven),
            '8' => Some(Digits::Eight),
            '9' => Some(Digits::Nine),
            _ => None
        }
    }

    pub fn find_first_and_last(input : &str) -> (Self, Self) {
        let mut found_first : Self = Digits::One;
        let mut found_last : Self = Digits::One;

        // Forward
        let mut acc = String::new();
        for c in input.chars() {
            match Digits::from_char(c) {
                Some(digit) => {
                    found_first = digit;
                    break;
                },
                _ => {
                    acc.push(c);
                    match Digits::contains(&acc) {
                        Some(digit) => {
                            found_first = digit;
                            break;
                        },
                        _ => (),
                    }
                }
            }
        }
        acc.clear();

        // Backward
        for c in input.chars().rev() {
            match Digits::from_char(c) {
                Some(digit) => {
                    found_last = digit;
                    break;
                },
                _ => {
                    acc.insert(0, c);
                    match Digits::contains(&acc) {
                        Some(digit) => {
                            found_last = digit;
                            break;
                        },
                        _ => (),
                    }
                }
            }
        }

        return (found_first.to_owned(), found_last.to_owned());
    }
}

pub fn run_part2() -> (bool, String) {
    let input = get_input("input.txt");
    //let input = get_input("input2_example.txt");
    let mut total_sum : i64 = 0;

    let lines = input.split("\n");

    for line in lines {
        let (first, last) = Digits::find_first_and_last(&line);
        let combined = format!("{}{}", first.as_char(), last.as_char());

        // println!("{} -> {} = {}", first.as_char(), last.as_char(), combined);
        total_sum += combined.parse::<i64>().unwrap();
    }

    return (
        true,
        format!("Day 1 part 2 => Total sum of first and last in each row is: {}", total_sum)
    );
}