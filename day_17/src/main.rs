use std::fs::read_to_string;

fn main() {
    let filename = String::from("puzzle_input.txt");
    let input = read_to_string(&filename).expect("Unable to read puzzle file");

    let containers: Vec<_> = input.lines().map(|v| v.parse::<u32>().unwrap()).collect();

    let mut combi_count = 0;
    for configuration in 0..=2u32.pow(containers.len() as u32) {
        let mut acc = 0;
        for (i, c) in containers.iter().enumerate() {
            if (configuration >> i) & 1 == 1 {
                acc += c;
            }
        }
        if acc == 150 {
            combi_count += 1;
        }
    }

    println!("Part One:");
    println!("Nb combinations: {}", combi_count);
}
