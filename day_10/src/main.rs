fn main() {
    let input = String::from("3113322113");
    let mut char_vec: Vec<_> = input.chars().collect();

    for _ in 0..40 {
        let mut cur_vec = Vec::new();
        let mut count = 1;
        for c in char_vec.windows(2) {
            if c[1] != c[0] {
                cur_vec.extend(count.to_string().chars());
                cur_vec.push(c[0]);
                count = 0;
            }
            count += 1;
        }
        cur_vec.extend(count.to_string().chars());
        cur_vec.push(*char_vec.last().unwrap());
        char_vec = cur_vec;
    }

    println!("Part One:");
    println!("  Result length: {}", char_vec.len());
}
