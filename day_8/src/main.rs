use std::fs::read_to_string;

fn main() {
    let filename = String::from("puzzle_input.txt");
    let input = read_to_string(&filename).expect("Unable to read the puzzle file.");

    let mut char_count = 0;
    for line in input.lines() {
        let line = line.strip_prefix("\"").unwrap();
        let line = line.strip_suffix("\"").unwrap();
        let mut sub_line = line.chars().as_str();
        let mut hex_count = 0;
        let mut quote_count = 0;
        let mut bs_count = 0;
        loop {
            let idx_hex = match sub_line.find("\\x") {
                Some(v) => v,
                None => 1000,
            };
            let idx_quote = match sub_line.find("\\\"") {
                Some(v) => v,
                None => 1000,
            };
            let idx_bs = match sub_line.find("\\\\") {
                Some(v) => v,
                None => 1000,
            };
            let idx;
            if idx_hex<idx_quote && idx_hex<idx_bs {
                idx = idx_hex+4;
                hex_count += 1;
            } else if idx_quote<idx_hex && idx_quote<idx_bs {
                idx = idx_quote+2;
                quote_count += 1;
            } else if idx_bs<idx_hex && idx_bs<idx_quote {
                idx = idx_bs+2;
                bs_count += 1;
            } else {
                idx = 0;
            }
            if sub_line.len() == 0 || idx == 0 {
                break;
            }
            sub_line = &sub_line[idx..];
        }
        char_count += 2 + 3*hex_count + quote_count + bs_count;
    }

    println!("Part One:");
    println!("  Total difference: {}", char_count);

    char_count = 0;
    for line in input.lines() {
        let orig_count = line.len();
        let line = line.escape_debug().to_string();
        char_count += line.len() + 2 - orig_count;
    }

    println!("Part Two:");
    println!("  Total difference: {}", char_count);
}
