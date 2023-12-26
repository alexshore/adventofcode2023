use num::integer::lcm;
use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt").trim_end();
    let _output = dbg!(part_two(input));
}

type RouteMap = HashMap<String, (String, String)>;
type Fork = (String, String);

fn parse_fork(input: &str) -> Fork {
    let (left, right) = input.split_once(", ").unwrap();
    (left.to_string(), right.to_string())
}

fn parse_input(input: &str) -> (String, RouteMap) {
    let (directions, routes) = input.split_once("\n\n").unwrap();

    let routes = routes
        .lines()
        .map(|line| {
            let (id, children) = line.split_once('=').unwrap();
            (id.trim().to_string(), (parse_fork(&children[2..10])))
        })
        .collect::<RouteMap>();

    (directions.to_string(), routes)
}

fn follow_direction(direction: &str, fork: &Fork) -> String {
    match direction {
        "L" => fork.0.clone(),
        "R" => fork.1.clone(),
        _ => unreachable!(),
    }
}

fn part_two(input: &str) -> u64 {
    let (directions, routes) = parse_input(input);

    let start_nodes = routes
        .keys()
        .filter(|&key| key.ends_with('A'))
        .map(|key| key.clone())
        .collect::<Vec<String>>();

    let mut steps = vec![0; start_nodes.len()];

    for (n_index, node) in start_nodes.iter().enumerate() {
        let mut current = node.clone();
        let mut index: usize = 0;
        let mut count: u64 = 0;

        while !current.ends_with('Z') {
            current = follow_direction(
                &directions.chars().nth(index).unwrap().to_string(),
                routes.get(&current).unwrap(),
            );
            count += 1;
            index += 1;
            if index >= directions.len() {
                index = 0
            }
        }

        steps[n_index] = count;
    }

    steps
        .into_iter()
        .reduce(|a, b| lcm(a.to_owned(), b.to_owned()))
        .unwrap()
}
