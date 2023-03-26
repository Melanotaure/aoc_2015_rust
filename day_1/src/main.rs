use std::fs::read_to_string;

fn main() {
    let filename = String::from("puzzle_input.txt");
    let file = read_to_string(&filename).expect("Unable to read puzzle file.");

    let floor = file.chars().fold(0, |acc, c| acc + match c { '(' => 1, ')' => -1, _ => 0, });

    println!("Part One:");
    println!("  Floor: {floor}");

    println!("\nPart Two:");
}
