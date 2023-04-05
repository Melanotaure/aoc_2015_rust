use std::fs::read_to_string;

struct Reindeer {
    name: String,
    flight_speed: u32,
    flight_duration: u32,
    rest_duration: u32,
    traveled_distance: u32,
    is_resting: bool,
    rest_tick: u32,
    flight_tick: u32,
    points: u32,
}

impl Reindeer {
    fn new(name: String, speed: u32, flight_duration: u32, rest_duration: u32) -> Self {
        Self {
            name: name,
            flight_speed: speed,
            flight_duration: flight_duration,
            rest_duration: rest_duration,
            traveled_distance: 0,
            is_resting: false,
            rest_tick: 0,
            flight_tick: 0,
            points: 0,
        }
    }
}

fn main() {
    let filename = String::from("puzzle_input.txt");
    let input = read_to_string(&filename).expect("Unable to read puzzle file.");

    let mut reindeers: Vec<Reindeer> = Vec::new();

    for line in input.lines() {
        let mut split_line = line.split(' ');
        let name = split_line.next().unwrap().to_string();
        let speed: u32 = split_line.nth(2).unwrap().parse().unwrap();
        let flight_duration: u32 = split_line.nth(2).unwrap().parse().unwrap();
        let rest_duration: u32 = split_line.nth(6).unwrap().parse().unwrap();
        reindeers.push(Reindeer::new(name, speed, flight_duration, rest_duration));
    }

    for _ in 1..=2503 {
        for reindeer in reindeers.iter_mut() {
            if reindeer.is_resting {
                reindeer.rest_tick += 1;
                if reindeer.rest_tick == reindeer.rest_duration {
                    reindeer.is_resting = false;
                    reindeer.rest_tick = 0;
                }
            } else {
                reindeer.flight_tick += 1;
                reindeer.traveled_distance += reindeer.flight_speed;
                if reindeer.flight_tick == reindeer.flight_duration {
                    reindeer.is_resting = true;
                    reindeer.flight_tick = 0;
                }
            }
        }
        reindeers.sort_by(|r1, r2| r2.traveled_distance.cmp(&r1.traveled_distance));
        let max_dist = reindeers[0].traveled_distance;
        reindeers[0].points += 1;
        for reindeer in reindeers.iter_mut().skip(1) {
            if reindeer.traveled_distance == max_dist {
                reindeer.points += 1;
            } else {
                break;
            }
        }
    }

    reindeers.sort_by(|r1, r2| r2.traveled_distance.cmp(&r1.traveled_distance));
    println!("Part One:");
    println!(
        "  The winner is {} with a distance of {} km.",
        reindeers[0].name, reindeers[0].traveled_distance
    );
    reindeers.sort_by(|r1, r2| r2.points.cmp(&r1.points));
    println!("Part Two:");
    println!(
        "  The winner is {} with {} points.",
        reindeers[0].name, reindeers[0].points
    );
}
