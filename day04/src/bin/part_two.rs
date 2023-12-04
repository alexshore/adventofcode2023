use std::collections::HashSet;

#[derive(Debug)]
struct ScratchCard {
    winning_nums: HashSet<u32>,
    my_nums: HashSet<u32>,
}

impl ScratchCard {
    fn new(winning_nums: HashSet<u32>, my_nums: HashSet<u32>) -> Self {
        ScratchCard {
            winning_nums,
            my_nums,
        }
    }
    fn count_matches(&self) -> u32 {
        self.winning_nums.intersection(&self.my_nums).count() as u32
    }
}

fn parse_set(set: &str) -> HashSet<u32> {
    set.split_whitespace()
        .map(|num| num.parse::<u32>().unwrap())
        .collect()
}

fn parse_line(line: &str) -> ScratchCard {
    let (_, line) = line.split_once(": ").unwrap();
    let (winning_nums, my_nums) = line.split_once(" | ").unwrap();
    ScratchCard::new(parse_set(winning_nums), parse_set(my_nums))
}

fn parse_data(data: &str) -> Vec<ScratchCard> {
    data.lines().map(|line| parse_line(line)).collect()
}

fn part_two(input: &str) -> u32 {
    let scratchcards = parse_data(input);

    let mut counts = vec![1; scratchcards.len()];
    for (i, card) in scratchcards.iter().enumerate() {
        for j in i + 1..i + 1 + card.count_matches() as usize {
            counts[j] += counts[i]
        }
    }

    counts.iter().sum()
}

fn main() {
    let input = include_str!("input.txt");
    let start = std::time::Instant::now();
    let _output = dbg!(part_two(input));
    println!("time taken: {:?}", start.elapsed());
}
