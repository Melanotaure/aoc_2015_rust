use std::{cmp::Ordering, fs::read_to_string};

fn main() {
    let filename = String::from("puzzle_input.txt");
    let input = read_to_string(&filename).expect("Unable to read puzzle file");

    let containers: Vec<_> = input.lines().map(|v| v.parse::<u32>().unwrap()).collect();

    let mut combo_count = 0;
    let mut min_container_number = std::u32::MAX;
    let mut min_combo_count = 0;
    for configuration in 0..=2u32.pow(containers.len() as u32) {
        let mut acc = 0;
        let mut container_number = 0;
        for (i, c) in containers.iter().enumerate() {
            if (configuration >> i) & 1 == 1 {
                acc += c;
                container_number += 1;
            }
        }
        if acc == 150 {
            combo_count += 1;
            match container_number.cmp(&min_container_number) {
                Ordering::Less => {
                    min_container_number = container_number;
                    min_combo_count = 1;
                }
                Ordering::Equal => min_combo_count += 1,
                Ordering::Greater => {}
            }
        }
    }

    println!("Part One:");
    println!("Nb combinations: {}", combo_count);
    println!("Part Two:");
    println!("Nb combinations: {}", min_combo_count);
}
