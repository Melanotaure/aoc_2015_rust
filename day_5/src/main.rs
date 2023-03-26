use std::fs::read_to_string;

fn main() {
    let filename = String::from("puzzle_input.txt");
    let input = read_to_string(&filename).expect("Unable to read puzzle file.");

    println!("Part One:");

    let mut nice_str_counter = 0;

    for line in input.lines() {
        if !(line.contains("ab") || line.contains("cd") || line.contains("pq") || line.contains("xy")) {
            let mut counter = 0;
            let mut prev_char = ' ';
            let mut pair_found = false;
            for c in line.chars() {
                if prev_char == c {
                    pair_found = true;
                } else {
                    prev_char = c;
                }
                match c {
                    'a' => counter += 1,
                    'e' => counter += 1,
                    'i' => counter += 1,
                    'o' => counter += 1,
                    'u' => counter += 1,
                    _ => counter += 0,
                }
            }
            if pair_found && counter >= 3 {
                nice_str_counter += 1;
            }
        }
    }
    println!("  There are {} nice strings.", nice_str_counter);

}
