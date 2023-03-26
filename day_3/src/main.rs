use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Hash, Debug, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x: x, y: y }
    }

    fn move_to(&mut self, dest: Point) {
        self.x += dest.x;
        self.y += dest.y;
    }
}

enum Turn {
    Santa,
    Robot,
}

fn main() {
    let filename = String::from("puzzle_input.txt");
    let input = read_to_string(&filename).expect("Unable to read puzzle file.");

    let mut cur_pos = Point::new(0, 0);
    let mut houses = HashMap::new();
    houses.insert(cur_pos, 1);
    for c in input.chars() {
        let delta = match c {
            '^' => Point::new(0, 1),
            'v' => Point::new(0, -1),
            '>' => Point::new(1, 0),
            '<' => Point::new(-1, 0),
            _ => Point::new(0, 0),
        };

        cur_pos.move_to(delta);
        houses.insert(cur_pos, 1);
    }

    println!("Part One:");
    println!("  Visited houses: {}", houses.len());

    println!("\nPart Two:");

    let mut santa_pos = Point::new(0, 0);
    let mut robot_pos = Point::new(0, 0);
    let mut cur_state = Turn::Santa;
    let mut houses = HashMap::new();
    houses.insert(santa_pos, 1);
    for c in input.chars() {
        let delta = match c {
            '^' => Point::new(0, 1),
            'v' => Point::new(0, -1),
            '>' => Point::new(1, 0),
            '<' => Point::new(-1, 0),
            _ => Point::new(0, 0),
        };

        match cur_state {
            Turn::Santa => {
                santa_pos.move_to(delta);
                houses.insert(santa_pos, 1);
                cur_state = Turn::Robot;
            }
            Turn::Robot => {
                robot_pos.move_to(delta);
                houses.insert(robot_pos, 1);
                cur_state = Turn::Santa;
            }
        };
    }

    println!("  Visited houses: {}", houses.len());
}
