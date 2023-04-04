fn main() {
    let pwd = "hepxcrrq";
    let mut pwd_vec: Vec<char> = pwd.chars().collect();
    increment_and_check(&mut pwd_vec);
    let pwd_str: String = pwd_vec.iter().collect();
    println!("Part One:");
    println!("  New pwd: {}", pwd_str);

    increment_and_check(&mut pwd_vec);
    let pwd_str: String = pwd_vec.iter().collect();
    println!("Part Two:");
    println!("  New new pwd: {}", pwd_str);
}

fn increment(v: char) -> (bool, char) {
    if v == 'z' {
        return (true, 'a');
    } else if v == 'h' || v == 'k' || v == 'n' {
        return (false, (v as u8 + 2) as char);
    } else {
        return (false, (v as u8 + 1) as char);
    }
}

fn check(v: &[char]) -> bool {
    let mut three_series_ok = false;
    let pair_ok: bool;
    let mut index = 0;
    let mut prev_pair = None;
    let mut pairs = 0;

    loop {
        if v[index] == v[index + 1] && Some(v[index]) != prev_pair {
            pairs += 1;
            prev_pair = Some(v[index]);
            index += 1;
        }
        index += 1;

        if pairs == 2 {
            pair_ok = true;
            break;
        } else if index >= v.len() - 1 {
            pair_ok = false;
            break;
        }
    }

    for w in v.windows(3) {
        if (w[2] as u8 == w[1] as u8 + 1) && (w[1] as u8 == w[0] as u8 + 1) {
            three_series_ok = true;
            break;
        }
    }
    pair_ok && three_series_ok
}

fn increment_and_check(vec: &mut [char]) {
    loop {
        // increment 8th letter
        let (next, v) = increment(vec[7]);
        vec[7] = v;
        if next {
            let (next, v) = increment(vec[6]);
            vec[6] = v;
            if next {
                let (next, v) = increment(vec[5]);
                vec[5] = v;
                if next {
                    let (next, v) = increment(vec[4]);
                    vec[4] = v;
                    if next {
                        let (next, v) = increment(vec[3]);
                        vec[3] = v;
                        if next {
                            let (next, v) = increment(vec[2]);
                            vec[2] = v;
                            if next {
                                let (next, v) = increment(vec[1]);
                                vec[1] = v;
                                if next {
                                    let (_next, v) = increment(vec[0]);
                                    vec[0] = v;
                                }
                            }
                        }
                    }
                }
            }
        }
        if check(vec) {
            break;
        }
    }
}