use std::collections::HashSet;

use pest::Parser;
use pest_derive::Parser;

fn main() {
    let input = include_str!("input.txt");
    let _output = dbg!(part_one(input));
}

#[grammar = "grammar.pest"]
struct CardParser {}

fn sum_card(card: &str) -> u32 {
    let mut card = CardParser::parse(Rule::card, card)
        .expect("Failed to parse.")
        .next()
        .unwrap()
        .into_inner();

    let set1: HashSet<&str> = card
        .next()
        .unwrap()
        .into_inner()
        .map(|pair| pair.as_str())
        .collect();

    let set2: HashSet<&str> = card
        .next()
        .unwrap()
        .into_inner()
        .map(|pair| pair.as_str())
        .collect();

    let intersection: u32 = set1.intersection(&set2).count() as u32 - 1;

    2_u32.pow(intersection)
}

fn part_one(input: &str) -> u32 {
    let mut total = 0;

    for line in input.lines() {
        total += sum_card(line.split(':').last().unwrap().trim())
    }

    total
}
