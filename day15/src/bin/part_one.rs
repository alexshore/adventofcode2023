fn main() {
    let input = include_str!("input.txt").trim_end();
    let _output = dbg!(part_one(input));
}

fn parse_input(input: &str) -> Vec<String> {
    input.split(',').map(|step| step.to_string()).collect()
}

fn calculate_hash(step: &str) -> u32 {
    let mut hash = 0;

    for char in step.chars() {
        hash += char as u32;
        hash *= 17;
        hash %= 256;
    }

    hash
}

fn part_one(input: &str) -> u32 {
    let sequence = parse_input(input);

    sequence
        .iter()
        .fold(0, |sum, step| sum + calculate_hash(step))
}
