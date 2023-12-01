use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt").trim_end();
    let _output = dbg!(part_two(input));
}

fn part_two(input: &str) -> u32 {
    let number_dict: HashMap<&str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let mut total: u32 = 0;

    for line in input.split('\n') {
        total += {
            let mut numbers = Vec::new();
            for i in 0..line.len() {
                for &number in number_dict.keys() {
                    if line[i..].starts_with(number) {
                        numbers.push(number_dict[number])
                    }
                }

                let c = line.chars().nth(i).unwrap();
                if c.is_ascii_digit() {
                    numbers.push(c.to_digit(10).unwrap())
                }
            }
            numbers.first().unwrap() * 10 + numbers.last().unwrap()
        }
    }

    total
}
