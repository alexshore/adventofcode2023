fn main() {
    let input = include_str!("input.txt").trim_end();
    let _output = dbg!(part_one(input));
}

fn parse_input(input: &str) -> Vec<Race> {
    let (times, records) = input.split_once('\n').unwrap();
    let (_, times) = times.split_once(':').unwrap();
    let (_, records) = records.split_once(':').unwrap();
    let records = records.split_ascii_whitespace().collect::<Vec<_>>();
    let times = times.split_ascii_whitespace().collect::<Vec<_>>();

    std::iter::zip(times, records)
        .map(|(time, record)| {
            Race::new(
                time.parse::<u32>().expect("failed to parse"),
                record.parse::<u32>().expect("failed to parse"),
            )
        })
        .collect::<Vec<Race>>()
}

struct Race {
    time: u32,
    record: u32,
}

impl Race {
    fn new(time: u32, record: u32) -> Race {
        Race { time, record }
    }

    fn calculate_ways(&self) -> u32 {
        let mut total = 0;

        for held_time in 1..self.time {
            if held_time * (self.time - held_time) > self.record {
                total += 1
            }
        }

        total
    }
}

fn part_one(input: &str) -> u32 {
    let races = parse_input(input);

    races.iter().map(|race| race.calculate_ways()).product()
}
