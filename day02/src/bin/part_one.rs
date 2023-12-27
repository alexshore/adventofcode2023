fn main() {
    let input = include_str!("input.txt").trim_end();
    let _output = dbg!(part_one(input));
}

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Colour {
    Blue,
    Red,
    Green,
}

impl Colour {
    fn from_str(input: &str) -> Self {
        match input {
            "blue" => Self::Blue,
            "red" => Self::Red,
            "green" => Self::Green,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Cube {
    quantity: u32,
    colour: Colour,
}

impl Cube {
    fn from_str(cube: &str) -> Self {
        let (quantity, colour) = cube.split_once(' ').unwrap();

        Cube {
            quantity: quantity.parse::<u32>().expect("failed to from_str"),
            colour: Colour::from_str(colour),
        }
    }
}

#[derive(Debug)]
struct Set {
    cubes: Vec<Cube>,
}

impl Set {
    fn from_str(set: &str) -> Self {
        let cubes = set.split(", ").map(|cube| Cube::from_str(cube)).collect();

        Set { cubes }
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<Set>,
}

impl Game {
    fn from_str(game: &str, id: u32) -> Self {
        let sets = game.split("; ").map(|set| Set::from_str(set)).collect();

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
        let (_, game) = line.split_once(": ").unwrap();
        let id: u32 = i as u32 + 1;
        games.push(Game::from_str(game, id));
    }

    for game in games {
        if game.is_in_bounds() {
            total += game.id;
        }
    }

    total
}
