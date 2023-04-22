use std::{collections::HashMap, fs::read_to_string};

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
                let val: u32 = v[v.len() - 1].parse().unwrap();
                let mut clue = v[v.len() - 2].to_string();
                clue.pop();
                clues.insert(clue, val);
            } else {
                break;
            }
        }
        sues.push(clues);
    }

    let mut remembered_clues: HashMap<String, u32> = HashMap::new();
    remembered_clues.insert("children".to_string(), 3);
    remembered_clues.insert("cats".to_string(), 7);
    remembered_clues.insert("samoyeds".to_string(), 2);
    remembered_clues.insert("pomeranians".to_string(), 3);
    remembered_clues.insert("akitas".to_string(), 0);
    remembered_clues.insert("vizslas".to_string(), 0);
    remembered_clues.insert("goldfish".to_string(), 5);
    remembered_clues.insert("trees".to_string(), 3);
    remembered_clues.insert("cars".to_string(), 2);
    remembered_clues.insert("perfumes".to_string(), 1);

    let mut scores: Vec<(usize, u32)> = Vec::new();
    for (index, sue) in sues.iter().enumerate() {
        let mut score: (usize, u32) = (index, 0);

        for (key, val) in remembered_clues.iter() {
            if sue.get_key_value(key).is_some() && sue[key] == *val {
                score.1 += 1;
            }
        }
        scores.push(score);
    }

    scores.sort_by(|a, b| b.1.cmp(&a.1));
    println!("Part Two:");
    println!("Aunt #{} is the one!", scores[0].0 + 1);
}
