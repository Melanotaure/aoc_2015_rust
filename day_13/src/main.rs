use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

use permutohedron::Heap;

fn main() {
    let filename = String::from("puzzle_input.txt");
    let input = read_to_string(&filename).expect("Unable to read puzzle file.");

    let mut names = HashSet::new();
    let mut guests_gains = HashMap::new();

    for line in input.lines() {
        let mut split_line = line.split(' ');

        let guest1 = split_line.next().unwrap().to_string();
        let gain = if split_line.nth(1).unwrap() == "gain" {
            1
        } else {
            -1
        };
        let val = gain * split_line.next().unwrap().parse::<i32>().unwrap();
        let mut guest2 = split_line.nth(6).unwrap().to_string();
        guest2.pop(); // remove the last character which is a point here

        names.insert(guest1.clone());
        names.insert(guest2.clone());
        let guest2_map = guests_gains.entry(guest1).or_insert_with(HashMap::new);
        guest2_map.insert(guest2, val);
    }

    let mut names: Vec<_> = names.iter().collect();
    let heap = Heap::new(&mut names);
    let mut max_h_level = 0;

    for perm in heap {
        let mut h_level = guests_gains[perm[0]][perm[perm.len() - 1]]
            + guests_gains[perm[perm.len() - 1]][perm[0]];
        let mut h_level_min = h_level;
        for pair in perm.windows(2) {
            let pair_h_level = guests_gains[pair[0]][pair[1]] + guests_gains[pair[1]][pair[0]];
            h_level_min = min(pair_h_level, h_level_min);
            h_level += pair_h_level;
        }
        max_h_level = max(h_level, max_h_level);
    }

    println!("Part One:");
    println!("  Max happiness: {}", max_h_level);
}
