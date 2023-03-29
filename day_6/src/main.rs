use std::fs::read_to_string;

const GRID_SIZE: usize = 1000;

fn coord_to_index(x: u32, y: u32) -> usize {
    x as usize * GRID_SIZE + y as usize
}

// fn index_to_coord(idx: usize) -> (u32, u32) {
//     ((idx / GRID_SIZE) as u32, (idx % GRID_SIZE) as u32)
// }

fn main() {
    let filename = String::from("puzzle_input.txt");
    let input = read_to_string(&filename).expect("Unable to read puzzle file.");

    let mut lights: Vec<bool> = vec![false; GRID_SIZE*GRID_SIZE];

    for line in input.lines() {
        let command: Vec<&str> = line.split_whitespace().collect();
        if command[0] == "toggle" {
            let start_coords: Vec<&str> = command[1].split(',').collect();
            let dest_coords: Vec<&str> = command[3].split(',').collect();
            let start_x: u32 = start_coords[0].parse().unwrap();
            let start_y: u32 = start_coords[1].parse().unwrap();
            let dest_x: u32 = dest_coords[0].parse().unwrap();
            let dest_y: u32 = dest_coords[1].parse().unwrap();
            for x in start_x..=dest_x {
                for y in start_y..=dest_y {
                    lights[coord_to_index(x, y)] ^= true;
                }
            }
        } else {
            let light_state = {
                if command[1] == "on" {
                    true
                } else {
                    false
                }
            };
            let start_coords: Vec<&str> = command[2].split(',').collect();
            let dest_coords: Vec<&str> = command[4].split(',').collect();
            let start_x: u32 = start_coords[0].parse().unwrap();
            let start_y: u32 = start_coords[1].parse().unwrap();
            let dest_x: u32 = dest_coords[0].parse().unwrap();
            let dest_y: u32 = dest_coords[1].parse().unwrap();
            for x in start_x..=dest_x {
                for y in start_y..=dest_y {
                    lights[coord_to_index(x, y)] = light_state;
                }
            }
        }
    }

    let mut light_on_counter: u32 = 0;
    for light in lights {
        if light {
            light_on_counter += 1;
        }
    }

    println!("Part One:");
    println!("  Lights on: {}", light_on_counter);
/*****************************************************************************/
    println!("\nPart Two:");
    let mut lights: Vec<u32> = vec![0; GRID_SIZE*GRID_SIZE];

    for line in input.lines() {
        let command: Vec<&str> = line.split_whitespace().collect();
        if command[0] == "toggle" {
            let start_coords: Vec<&str> = command[1].split(',').collect();
            let dest_coords: Vec<&str> = command[3].split(',').collect();
            let start_x: u32 = start_coords[0].parse().unwrap();
            let start_y: u32 = start_coords[1].parse().unwrap();
            let dest_x: u32 = dest_coords[0].parse().unwrap();
            let dest_y: u32 = dest_coords[1].parse().unwrap();
            for x in start_x..=dest_x {
                for y in start_y..=dest_y {
                    lights[coord_to_index(x, y)] += 2;
                }
            }
        } else {
            let start_coords: Vec<&str> = command[2].split(',').collect();
            let dest_coords: Vec<&str> = command[4].split(',').collect();
            let start_x: u32 = start_coords[0].parse().unwrap();
            let start_y: u32 = start_coords[1].parse().unwrap();
            let dest_x: u32 = dest_coords[0].parse().unwrap();
            let dest_y: u32 = dest_coords[1].parse().unwrap();
            for x in start_x..=dest_x {
                for y in start_y..=dest_y {
                    if command[1] == "on" {
                        lights[coord_to_index(x, y)] += 1;
                    } else {
                        lights[coord_to_index(x, y)] = match lights[coord_to_index(x, y)].checked_sub(1) {
                            Some(v) => v,
                            None => 0,
                        };
                    }
                }
            }
        }
    }

    let brightness: u32 = lights.iter().sum();
    println!("  Total brightness: {}", brightness);
}
