fn main() {
    let input = include_str!("input.txt").trim_end();
    let _output = dbg!(part_two(input));
}

fn parse_input(input: &str) -> Race {
    let (times, records) = input.split_once('\n').unwrap();
    let (_, times) = times.split_once(':').unwrap();
    let (_, records) = records.split_once(':').unwrap();
    let time = times.split_ascii_whitespace().collect::<String>();
    let record = records.split_ascii_whitespace().collect::<String>();

    Race::new(
        time.parse::<u64>().expect("failed to parse"),
        record.parse::<u64>().expect("failed to parse"),
    )
}

struct Race {
    time: u64,
    record: u64,
}

impl Race {
    fn new(time: u64, record: u64) -> Race {
        Race { time, record }
    }

    fn calculate_ways(&self) -> u64 {
        let mut total = 0;

        for held_time in 1..self.time {
            if held_time * (self.time - held_time) > self.record {
                total += 1
            }
        }

        total
    }
}

fn part_two(input: &str) -> u64 {
    let race = parse_input(input);

    race.calculate_ways()
}
