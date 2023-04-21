use std::{fs::read_to_string, collections::HashMap};

fn main() {
    let filename = String::from("puzzle_input.txt");
    let input = read_to_string(&filename).expect("Unable to read puzzle file.");

    let mut sues = Vec::new();
    for line in input.lines() {
        let mut clues = HashMap::new();
        let mut split_line = line.split(',');
        loop {
            if let Some(clue_val) = split_line.nth_back(0) {
                let v: Vec<_> = clue_val.split_whitespace().collect();
                let val: u32 = v[v.len()-1].parse().unwrap();
                let mut clue = v[v.len()-2].to_string();
                clue.pop();
                clues.insert(clue, val);
            } else {
                break;
            }
        }
        sues.push(clues);
    }

    for sue in sues {
        println!("{:?}", sue);
    }
}
