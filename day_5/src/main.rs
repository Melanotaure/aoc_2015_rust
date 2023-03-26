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
                    'a' | 'e' | 'i' | 'o' | 'u' => counter += 1,
                    _ => counter += 0,
                }
            }
            if pair_found && counter >= 3 {
                nice_str_counter += 1;
            }
        }
    }
    println!("  There are {} nice strings.", nice_str_counter);

    println!("Part Two:");
    nice_str_counter = 0;
    for line in input.lines() {
        let mut prev_c = ' ';
        let mut pair_found = false;
        for c in line.chars() {
            let pair = format!("{}{}", prev_c, c);
            let pair_count = line.matches(pair.as_str()).count();
            if pair_count >=2 {
                pair_found = true;
                break;
            }
            prev_c = c;
        }

        let mut first_c = ' ';
        let mut second_c = ' ';
        let mut triplet_found = false;
        for c in line.chars() {
            if c == first_c {
                triplet_found = true;
                break;
            } else {
                first_c = second_c;
                second_c = c;
            }
        }
        if pair_found && triplet_found {
            nice_str_counter += 1;
        }
    }

    println!("  There are {} nice strings.", nice_str_counter);
}
