use crate::aoc_utility::aoc_utility;

struct Digit {
    x: i64,
    y: i64,
    index: i64,
    length: i64,
    digit: i64
}

struct Symbol {
    x: i64,
    y: i64,
    index: i64,
    symbol: char
}

struct Schematic {
    data: String,
    digits: Vec<Digit>,
    symbols: Vec<Symbol>,
    column_width: i64
}

fn index_to_coord2d(index: i64, column_width: i64) -> (i64, i64) {
    return (
        index % column_width,
        index / column_width
    );
}

impl Schematic {
    fn parse(data: &str) -> Self {

        let mut schematic = Self {
            data: data.to_owned(),
            digits: Vec::new(),
            symbols: Vec::new(),
            column_width: data.find('\n').unwrap() as i64
        };

        let mut accumilator = String::from("");
        schematic.data.chars()
            .enumerate()
            .for_each(|(ind, c)| {

                if c.is_numeric() {
                    accumilator.push(c);
                } else {

                    if accumilator.len() > 0 {
                        let index = ind as i64 - accumilator.len() as i64;
                        let (cx, cy) = index_to_coord2d(index, schematic.column_width);
                        schematic.digits.push(
                            Digit {
                                x: cx,
                                y: cy,
                                index: index,
                                length: accumilator.len() as i64,
                                digit: schematic.data[index as usize..(index as usize + accumilator.len())].parse::<i64>().unwrap()
                            }
                        );
                        accumilator.clear();
                    } 
                    
                    if c != '.' && !c.is_numeric() && c != '\n' && c != '\r' && c != '\t' {
                        let index = ind as i64;
                        let (cx, cy) = index_to_coord2d(index, schematic.column_width);
                        schematic.symbols.push( Symbol {
                            x: cx,
                            y: cy,
                            index: index,
                            symbol: schematic.data[index as usize..(index + 1) as usize].chars().next().unwrap()
                        });
                    }
                }
            });

        return schematic;
    }
}

pub fn run_part1() -> (bool, String) {
    let input = aoc_utility::get_input("input_example.txt", "day_3");
    let schematic = Schematic::parse(&input);

    return (false, format!("NOT IMPLEMENTED"));
}