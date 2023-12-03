fn main() {
    let input = include_str!("input.txt").trim_end();
    let _output = dbg!(part_one(input));
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn is_adjacent(&self, points: &[Point]) -> bool {
        for dx in -1..=1 {
            for dy in -1..=1 {
                if points.contains(&Point {
                    x: self.x + dx,
                    y: self.y + dy,
                }) {
                    return true;
                }
            }
        }
        false
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Number {
    value: u32,
    locs: Vec<Point>,
}

impl Number {
    fn new(value: String, locs: Vec<Point>) -> Self {
        Number {
            value: value.parse().unwrap(),
            locs,
        }
    }
}

fn parse_symbols(schematic: &[&str]) -> Vec<Point> {
    let mut symbols = Vec::new();

    for (x, &row) in schematic.iter().enumerate() {
        for (y, c) in row.chars().enumerate() {
            if !c.is_ascii_digit() && c != '.' {
                symbols.push(Point {
                    x: x as i32,
                    y: y as i32,
                })
            }
        }
    }

    symbols
}

fn parse_numbers(schematic: &[&str]) -> Vec<Number> {
    let mut numbers = Vec::new();

    let mut current_value = String::new();
    let mut current_locs = Vec::new();

    for (x, &row) in schematic.iter().enumerate() {
        for (y, c) in row.chars().enumerate() {
            match c {
                d if d.is_ascii_digit() => {
                    current_value.push(d);
                    current_locs.push(Point {
                        x: x as i32,
                        y: y as i32,
                    })
                }
                _ if !current_value.is_empty() => {
                    numbers.push(Number::new(current_value.clone(), current_locs.clone()));
                    current_value.clear();
                    current_locs.clear()
                }
                _ => (),
            }
        }
        if !current_value.is_empty() {
            numbers.push(Number::new(current_value.clone(), current_locs.clone()));
        }
        current_value.clear();
        current_locs.clear();
    }
    numbers
}

fn part_one(input: &str) -> u32 {
    let schematic: Vec<&str> = input.split('\n').collect();
    let mut total = 0;

    let numbers = parse_numbers(&schematic);
    let symbols = parse_symbols(&schematic);

    'outer: for number in numbers {
        for loc in number.locs {
            if loc.is_adjacent(&symbols) {
                total += number.value;
                continue 'outer;
            }
        }
    }

    total
}
