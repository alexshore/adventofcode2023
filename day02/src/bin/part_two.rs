use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt").trim_end();
    let _output = dbg!(part_two(input));
}

// code :3

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
    sets: Vec<Set>,
}

impl Game {
    fn parse(game: String) -> Self {
        let sets = game
            .split("; ")
            .map(|set| Set::parse(set.to_owned()))
            .collect();

        Game { sets }
    }

    fn min_cubes(&self) -> u32 {
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;

        for set in self.sets.iter() {
            for cube in set.cubes.iter() {
                match *cube {
                    Cube {
                        quantity,
                        colour: Colour::Red,
                    } if quantity > min_red => min_red = quantity,
                    Cube {
                        quantity,
                        colour: Colour::Green,
                    } if quantity > min_green => min_green = quantity,
                    Cube {
                        quantity,
                        colour: Colour::Blue,
                    } if quantity > min_blue => min_blue = quantity,
                    _ => (),
                }
            }
        }

        min_red * min_green * min_blue
    }
}

fn part_two(input: &str) -> u32 {
    let mut total = 0;

    let mut games: Vec<Game> = Vec::new();

    for line in input.split('\n') {
        let game = line.split(": ").collect::<Vec<_>>()[1];
        games.push(Game::parse(game.to_owned()));
    }

    for game in games {
        total += game.min_cubes();
    }

    total
}
