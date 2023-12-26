fn main() {
    let input = include_str!("input.txt").trim_end();
    let _output = dbg!(part_two(input));
}

fn parse_input(input: &str) -> Vec<String> {
    input.split(',').map(|step| step.to_string()).collect()
}

#[derive(Clone, Debug)]
struct Lens {
    label: String,
    focal_length: usize,
}

#[derive(Clone, Debug)]
struct Box {
    contents: Vec<Lens>,
}

impl Box {
    fn new() -> Self {
        Self {
            contents: Vec::new(),
        }
    }

    fn calc_power(&self) -> usize {
        self.contents
            .iter()
            .enumerate()
            .map(|(i, lens)| (i + 1) * lens.focal_length)
            .sum()
    }
}

fn calculate_hash(label: &str) -> usize {
    let mut hash = 0;

    for char in label.chars() {
        hash += char as usize;
        hash *= 17;
        hash %= 256;
    }

    hash
}

fn do_step(step: &str, boxes: &mut [Box]) {
    let (label, focal_length) = step.split_once(['-', '=']).unwrap();
    let action = step.chars().find(|c| !c.is_alphanumeric()).unwrap();

    let box_index = calculate_hash(label);

    match action {
        '-' => {
            if let Some(pos) = boxes[box_index]
                .contents
                .iter()
                .position(|lens| lens.label == label)
            {
                boxes[box_index].contents.remove(pos);
            }
        }
        '=' => {
            if let Some(pos) = boxes[box_index]
                .contents
                .iter()
                .position(|lens| lens.label == label)
            {
                boxes[box_index].contents[pos] = Lens {
                    label: label.to_string(),
                    focal_length: focal_length.parse::<usize>().expect("failed to parse"),
                }
            } else {
                boxes[box_index].contents.push(Lens {
                    label: label.to_string(),
                    focal_length: focal_length.parse::<usize>().expect("failed to parse"),
                })
            }
        }
        _ => unreachable!(),
    }
}

fn part_two(input: &str) -> usize {
    let sequence = parse_input(input);
    let mut boxes: Vec<Box> = vec![Box::new(); 256];

    for step in sequence.iter() {
        do_step(step, &mut boxes)
    }

    boxes
        .iter()
        .enumerate()
        .map(|(i, b_ox)| (i + 1) * b_ox.calc_power())
        .sum()
}
