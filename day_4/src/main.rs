use md5;

const SECRET_KEY: &str = "iwrupvqb";

fn main() {
    let mut counter: i64 = 0;

    loop {
        let advent_coin = SECRET_KEY.to_string() + format!("{counter}").as_str();
        let digest = md5::compute(advent_coin.clone());

        let digest = format!("{:x}", digest);
        if digest.starts_with("00000") {
            break;
        }
        counter += 1;
    }

    println!("Part One:");
    println!("  value: {}", counter);

    println!("\nPart Two:");
    counter = 0;
    loop {
        let advent_coin = SECRET_KEY.to_string() + format!("{counter}").as_str();
        let digest = md5::compute(advent_coin.clone());

        let digest = format!("{:x}", digest);
        if digest.starts_with("000000") {
            break;
        }
        counter += 1;
    }
    println!("  value: {}", counter);
}
