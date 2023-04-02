use std::{collections::HashMap, fs::read_to_string};

use permutohedron::Heap;

fn main() {
    let filename = String::from("puzzle_input.txt");
    let input = read_to_string(&filename).expect("Unable to read puzzle file.");

    let mut city_list = Vec::new();
    let mut dist_map = HashMap::new();

    for line in input.lines() {
        let mut line_split = line.split(" = ");
        let split_cities = line_split.next().unwrap().split(" to ");
        let mut city_pair = split_cities
            .map(std::string::ToString::to_string)
            .collect::<Vec<_>>();
        city_pair.sort();
        city_list.extend(city_pair.clone());
        let dist = line_split.next().unwrap();
        dist_map.insert(city_pair, dist.parse::<u32>().unwrap());
    }

    city_list.sort();
    city_list.dedup();
    let heap = Heap::new(&mut city_list);
    let mut min_dist = u32::max_value();
    let mut max_dist = 0;
    for perm in heap {
        let mut cur_dist = 0;
        for city_slice in perm.windows(2) {
            let mut city_pair = city_slice.to_vec();
            city_pair.sort();
            cur_dist += dist_map[&city_pair];
        }
        if cur_dist < min_dist {
            min_dist = cur_dist;
        }
        if cur_dist > max_dist {
            max_dist = cur_dist;
        }
    }
    println!("Part One:");
    println!("  Shortest route: {}", min_dist);
    println!("Part Two:");
    println!("  Longest route: {}", max_dist);
}
