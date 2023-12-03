use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt").trim_end();
    let _output = dbg!(part_one(input));
}

// code :3

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Colour {
    Blue,
    Red,
    Green,
}

#[derive(Debug)]
struct Cube {
    quantity: u32,
    colour: Colour,
}

impl Cube {
    fn parse(cube: String) -> Self {
        let split_data: Vec<&str> = cube.split(' ').collect();

        let colour_map = HashMap::from([
            ("blue", Colour::Blue),
            ("red", Colour::Red),
            ("green", Colour::Green),
        ]);

        Cube {
            quantity: split_data.first().unwrap().parse::<u32>().unwrap(),
            colour: colour_map[split_data.last().unwrap()],
        }
    }
}

#[derive(Debug)]
struct Set {
    cubes: Vec<Cube>,
}

impl Set {
    fn parse(set: String) -> Self {
        let cubes = set
            .split(", ")
            .map(|cube| Cube::parse(cube.to_owned()))
            .collect();

        Set { cubes }
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<Set>,
}

impl Game {
    fn parse(game: String, id: u32) -> Self {
        let sets = game
            .split("; ")
            .map(|set| Set::parse(set.to_owned()))
            .collect();

        Game { sets, id }
    }

    fn is_in_bounds(&self) -> bool {
        for set in self.sets.iter() {
            for cube in set.cubes.iter() {
                if cube.colour == Colour::Red && cube.quantity > MAX_RED {
                    return false;
                }

                if cube.colour == Colour::Green && cube.quantity > MAX_GREEN {
                    return false;
                }

                if cube.colour == Colour::Blue && cube.quantity > MAX_BLUE {
                    return false;
                }
            }
        }
        true
    }
}

fn part_one(input: &str) -> u32 {
    let mut total = 0;

    let mut games: Vec<Game> = Vec::new();

    for (i, line) in input.split('\n').enumerate() {
        let game = line.split(": ").collect::<Vec<_>>()[1];
        let id: u32 = i as u32 + 1;
        games.push(Game::parse(game.to_owned(), id));
    }

    for game in games {
        if game.is_in_bounds() {
            total += game.id;
        }
    }

    total
}
