#![allow(dead_code)]

use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt").trim_end();
    let _output = dbg!(part_one(input));
}

type Grid = Vec<Vec<char>>;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Config(Position, Direction);

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum Direction {
    EAST,
    WEST,
    NORTH,
    SOUTH,
}

impl Direction {
    fn get_move(&self) -> (i32, i32) {
        match self {
            Direction::EAST => (0, 1),
            Direction::WEST => (0, -1),
            Direction::NORTH => (-1, 0),
            Direction::SOUTH => (1, 0),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Position(usize, usize);

impl Position {
    fn try_move(&mut self, grid: &Grid, dir: &Direction) -> bool {
        let move_ = Direction::get_move(dir);

        let new_x = move_.1 + self.1 as i32;
        let new_y = move_.0 + self.0 as i32;

        if !Position::in_bounds(new_x, new_y, grid) {
            false
        } else {
            self.0 = new_y as usize;
            self.1 = new_x as usize;
            true
        }
    }

    fn in_bounds(x: i32, y: i32, grid: &Grid) -> bool {
        x >= 0 && y >= 0 && x < grid[0].len() as i32 && y < grid.len() as i32
    }
}

fn parse_input(input: &str) -> Grid {
    input
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect()
}

fn print_output(energized: &HashSet<Position>, grid: &Grid) {
    for row in 0..grid.len() {
        let mut s_row = String::new();

        for col in 0..grid[0].len() {
            if energized.contains(&Position(row, col)) {
                s_row.push('#')
            } else {
                s_row.push('.')
            }
        }

        println!("{}", s_row)
    }
}

fn part_one(input: &str) -> usize {
    let grid: Grid = parse_input(input);

    let mut energized = HashSet::new();

    let mut starting_configs = HashSet::new();
    starting_configs.insert(Config(Position(0, 0), Direction::EAST));

    let mut used_configs = HashSet::new();

    while !starting_configs.is_empty() {
        let config = starting_configs.iter().last().unwrap().to_owned();
        starting_configs.remove(&config);
        if used_configs.contains(&config) {
            continue;
        }
        used_configs.insert(config.clone());

        let mut current_energized = HashSet::new();

        let Config(mut pos, mut dir) = config;

        loop {
            if !current_energized.insert(Config(pos.clone(), dir.clone())) {
                current_energized.into_iter().for_each(|c| {
                    energized.insert(c.0.clone());
                });
                break;
            }

            match grid[pos.0][pos.1] {
                '/' => match dir {
                    Direction::EAST => dir = Direction::NORTH,
                    Direction::WEST => dir = Direction::SOUTH,
                    Direction::NORTH => dir = Direction::EAST,
                    Direction::SOUTH => dir = Direction::WEST,
                },
                '\\' => match dir {
                    Direction::EAST => dir = Direction::SOUTH,
                    Direction::WEST => dir = Direction::NORTH,
                    Direction::NORTH => dir = Direction::WEST,
                    Direction::SOUTH => dir = Direction::EAST,
                },
                '|' if (dir == Direction::EAST) | (dir == Direction::WEST) => {
                    let new_config = Config(pos.clone(), Direction::NORTH);
                    starting_configs.insert(new_config);
                    dir = Direction::SOUTH
                }
                '-' if (dir == Direction::NORTH) | (dir == Direction::SOUTH) => {
                    let new_config = Config(pos.clone(), Direction::WEST);
                    starting_configs.insert(new_config);
                    dir = Direction::EAST
                }
                _ => (),
            }

            if !pos.try_move(&grid, &dir) {
                current_energized.into_iter().for_each(|c| {
                    energized.insert(c.0.clone());
                    ()
                });
                break;
            }
        }
    }

    print_output(&energized, &grid);

    energized.len()
}
