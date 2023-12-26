#![allow(dead_code)]

fn main() {
    let input = include_str!("input.txt").trim_end();
    let _output = dbg!(part_one(input));
}

enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn from(str_dir: &str) -> Self {
        match str_dir {
            "U" => Direction::North,
            "R" => Direction::East,
            "D" => Direction::South,
            "L" => Direction::West,
            _ => unreachable!(),
        }
    }
}

struct Edge {
    direction: Direction,
    distance: u32,
}

#[derive(Debug, Clone, Copy)]
struct Position(i32, i32);

fn parse_input(input: &str) -> Vec<Edge> {
    input
        .lines()
        .map(|line| {
            let mut split_line = line.split_ascii_whitespace();

            let dir = split_line.next().unwrap();
            let dist = split_line.next().unwrap();

            Edge {
                direction: Direction::from(dir),
                distance: dist.parse::<u32>().expect("failed to parse"),
            }
        })
        .collect()
}

fn calc_vertices(edges: &[Edge]) -> Vec<Position> {
    let mut res = vec![Position(0, 0)];
    for edge in edges {
        let last_pos = res.last().unwrap();
        res.push(match edge.direction {
            Direction::North => Position(last_pos.0, last_pos.1 + edge.distance as i32),
            Direction::East => Position(last_pos.0 + edge.distance as i32, last_pos.1),
            Direction::South => Position(last_pos.0, last_pos.1 - edge.distance as i32),
            Direction::West => Position(last_pos.0 - edge.distance as i32, last_pos.1),
        })
    }
    res.reverse();
    res
}

fn calc_trench_length(edges: &[Edge]) -> u32 {
    edges.iter().map(|edge| edge.distance).sum()
}

fn calc_area(vertices: &[Position]) -> u32 {
    let mut accumulation = 0;

    for window in vertices.windows(2) {
        let &[a, b] = window else { unreachable!() };
        accumulation += (a.0 * b.1) - (a.1 * b.0)
    }

    accumulation as u32 / 2
}

fn part_one(input: &str) -> u32 {
    let edges = parse_input(input);
    let trench_length = calc_trench_length(&edges);

    let mut vertices = calc_vertices(&edges);
    vertices.push(vertices[0]);

    trench_length + (calc_area(&vertices) - (trench_length / 2) + 1)
}
