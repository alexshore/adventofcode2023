#![allow(dead_code)]

fn main() {
    let input = include_str!("input.txt").trim_end();
    let _output = dbg!(part_two(input));
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
            "0" => Direction::East,
            "1" => Direction::South,
            "2" => Direction::West,
            "3" => Direction::North,
            _ => unreachable!(),
        }
    }
}

struct Edge {
    direction: Direction,
    distance: u64,
}

#[derive(Debug, Clone, Copy)]
struct Position(i64, i64);

fn parse_hex(hex_rep: &str) -> (Direction, u64) {
    let to_remove: &[_] = &['#', '(', ')'];
    let (dist, dir) = hex_rep.trim_matches(to_remove).split_at(5);

    (
        Direction::from(dir),
        u64::from_str_radix(dist, 16).expect("failed to parse"),
    )
}

fn parse_input(input: &str) -> Vec<Edge> {
    input
        .lines()
        .map(|line| {
            let (_, hex_rep) = line.rsplit_once(' ').unwrap();

            let (dir, dist) = parse_hex(hex_rep);

            Edge {
                direction: dir,
                distance: dist,
            }
        })
        .collect()
}

fn calc_vertices(edges: &[Edge]) -> Vec<Position> {
    let mut res = vec![Position(0, 0)];
    for edge in edges {
        let last_pos = res.last().unwrap();
        res.push(match edge.direction {
            Direction::North => Position(last_pos.0, last_pos.1 + edge.distance as i64),
            Direction::East => Position(last_pos.0 + edge.distance as i64, last_pos.1),
            Direction::South => Position(last_pos.0, last_pos.1 - edge.distance as i64),
            Direction::West => Position(last_pos.0 - edge.distance as i64, last_pos.1),
        })
    }
    res.reverse();
    res
}

fn calc_trench_length(edges: &[Edge]) -> u64 {
    edges.iter().map(|edge| edge.distance).sum()
}

fn calc_area(vertices: &[Position]) -> u64 {
    let mut accumulation = 0;

    for window in vertices.windows(2) {
        let &[a, b] = window else { unreachable!() };
        accumulation += (a.0 * b.1) - (a.1 * b.0)
    }

    accumulation as u64 / 2
}

fn part_two(input: &str) -> u64 {
    let edges = parse_input(input);
    let trench_length = calc_trench_length(&edges);

    let mut vertices = calc_vertices(&edges);
    vertices.push(vertices[0]);

    trench_length + (calc_area(&vertices) - (trench_length / 2) + 1)
}
