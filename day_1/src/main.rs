use std::fs::read_to_string;

fn main() {
    let filename = String::from("puzzle_input.txt");
    let input = read_to_string(&filename).expect("Unable to read puzzle file.");

    println!("Part One:");
    let floor = input.chars().fold(0, |acc, c| acc + match c { '(' => 1, ')' => -1, _ => 0, });
    println!("  Floor: {floor}");

    println!("\nPart Two:");
    let mut acc = 0;
    let mut position = 0;
    for (pos, c) in input.chars().enumerate() {
        acc += match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };
        if acc == -1 {
            position = pos + 1;
            break;
        }
    }
    println!("  Enter basement at: {position}");
}
