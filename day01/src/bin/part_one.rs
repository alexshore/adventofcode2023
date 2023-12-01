fn main() {
    let input = include_str!("input.txt").trim_end();
    let _output = dbg!(part_one(input));
}

fn part_one(input: &str) -> u32 {
    let mut total = 0;

    for line in input.split('\n') {
        total += {
            let numbers = line
                .chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<Vec<_>>();

            numbers.first().unwrap().to_digit(10).unwrap() * 10
                + numbers.last().unwrap().to_digit(10).unwrap()
        };
    }

    total
}
