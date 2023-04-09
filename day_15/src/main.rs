use std::fs::read_to_string;
use std::str::FromStr;
use std::cmp::max;

fn main() {
    let filename = String::from("puzzle_input.txt");
    let input = read_to_string(&filename).expect("Unable to read puzzle file.");

    let mut ingredients = Vec::new();
    for line in input.lines() {
        let properties: Vec<_> = line
            .split(':')
            .nth(1)
            .unwrap()
            .split(',')
            .map(|s| i32::from_str(s.split(' ').rev().next().unwrap()).unwrap())
            .collect();
        ingredients.push(properties);
    }

    let mut score_max = 0;
    let mut score_max_500_cal = 0;
    let mut quantities = vec![0; ingredients.len()];
    for i in 0..=usize::pow(101, ingredients.len() as u32) {
        for (idx, qty) in quantities.iter_mut().enumerate() {
            *qty = (i as u32 / 101_u32.pow(idx as u32)) % 101;
        }
        if quantities.iter().sum::<u32>() > 100 {
            continue;
        };
        let mix = (0usize..5)
            .map(|q_idx| {
                ingredients
                    .iter()
                    .enumerate()
                    .map(|(idx, ing)| ing[q_idx] * quantities[idx] as i32)
                    .sum::<i32>()
            })
            .map(|sum| if sum < 0 { 0 } else { sum })
            .collect::<Vec<_>>();
        let score: i32 = mix[0..4].iter().product();
        score_max = max(score, score_max);
        if mix[4] == 500 {
            score_max_500_cal = max(score, score_max_500_cal);
        }
    }
    // /!\ to be run in release mode
    println!("Part One:");
    println!("  highest-scoring cookie: {score_max}");
    println!("Part Two:");
    println!("  highest-scoring cookie with 500 cal: {}", score_max_500_cal);
}
