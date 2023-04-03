fn main() {
    let input = String::from("3113322113");
    let mut char_vec: Vec<_> = input.chars().collect();

    for nb_times in 0..50 {
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

        if nb_times == 39 {
            println!("Part One: [for 40 iterations]");
            println!("  Result length: {}", char_vec.len());
        }
    }

    println!("Part Two: [for 50 iterations]");
    println!("  Result length: {}", char_vec.len());
}
