#![allow(dead_code)]

use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt").trim_end();
    let _output = dbg!(part_two(input));
}

type Grid = Vec<Vec<char>>;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Config(Position, Direction);

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum Direction {
    East,
    West,
    North,
    South,
}

impl Direction {
    fn get_move(&self) -> (i32, i32) {
        match self {
            Direction::East => (0, 1),
            Direction::West => (0, -1),
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
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

fn get_starting_configs(grid: &Grid) -> Vec<Config> {
    let mut res = Vec::new();

    for i in 0..grid.len() {
        res.push(Config(Position(0, i), Direction::South));
        res.push(Config(Position(i, 0), Direction::East));
        res.push(Config(Position(grid.len() - 1, i), Direction::North));
        res.push(Config(Position(i, grid.len() - 1), Direction::West));
    }

    res
}

fn calc_energized(grid: &Grid, config: Config) -> usize {
    let mut energized: HashSet<Position> = HashSet::new();

    let mut starting_configs: HashSet<Config> = HashSet::new();
    starting_configs.insert(config);

    let mut used_configs: HashSet<Config> = HashSet::new();

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
                    Direction::East => dir = Direction::North,
                    Direction::West => dir = Direction::South,
                    Direction::North => dir = Direction::East,
                    Direction::South => dir = Direction::West,
                },
                '\\' => match dir {
                    Direction::East => dir = Direction::South,
                    Direction::West => dir = Direction::North,
                    Direction::North => dir = Direction::West,
                    Direction::South => dir = Direction::East,
                },
                '|' => match dir {
                    Direction::East | Direction::West => {
                        let new_config = Config(pos.clone(), Direction::North);
                        starting_configs.insert(new_config);
                        dir = Direction::South
                    }
                    _ => (),
                },
                '-' => match dir {
                    Direction::North | Direction::South => {
                        let new_config = Config(pos.clone(), Direction::West);
                        starting_configs.insert(new_config);
                        dir = Direction::East
                    }
                    _ => (),
                },
                _ => (),
            }

            if !pos.try_move(grid, &dir) {
                current_energized.into_iter().for_each(|c| {
                    energized.insert(c.0.clone());
                });
                break;
            }
        }
    }

    energized.len()
}

fn part_two(input: &str) -> usize {
    let grid: Grid = parse_input(input);

    let starting_configs = get_starting_configs(&grid);

    starting_configs
        .iter()
        .map(|config| calc_energized(&grid, config.clone()))
        .max()
        .unwrap()
}
