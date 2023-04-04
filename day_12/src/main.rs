use std::fs::read_to_string;
use serde_json::Value;


fn main() {
    let filename = String::from("puzzle_input.txt");
    let input = read_to_string(&filename).expect("Unable to read puzzle file.");

    let my_json: Value = serde_json::from_str(&input).unwrap();

    println!("Part One:");
    println!("  Total: {}", json_parse(0, &my_json));
}

fn json_parse(accu: i64, json: &Value) -> i64 {
    match json {
        Value::Number(num) => accu + num.as_i64().unwrap(),
        Value::Array(ref a) => a.iter().fold(accu, |sum, val| json_parse(sum, val)),
        Value::Object(ref obj) => obj.values().fold(accu, |sum, val| json_parse(sum, val)),
        _ => accu,
    }
}