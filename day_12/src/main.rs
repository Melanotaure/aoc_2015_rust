use serde_json::{Map, Value};
use std::fs::read_to_string;

fn main() {
    let filename = String::from("puzzle_input.txt");
    let input = read_to_string(&filename).expect("Unable to read puzzle file.");

    let my_json: Value = serde_json::from_str(&input).unwrap();

    println!("Part One:");
    println!("  Total: {}", json_parse(0, &my_json, true));
    println!("Part Two: [without 'red' entries]");
    println!("  Total: {}", json_parse(0, &my_json, false));
}

fn json_parse(accu: i64, json: &Value, count_red: bool) -> i64 {
    match json {
        Value::Number(num) => accu + num.as_i64().unwrap(),
        Value::Array(ref a) => a
            .iter()
            .fold(accu, |sum, val| json_parse(sum, val, count_red)),
        Value::Object(ref obj) if count_red || !has_red(obj) => obj
            .values()
            .fold(accu, |sum, val| json_parse(sum, val, count_red)),
        _ => accu,
    }
}

fn has_red(obj: &Map<String, Value>) -> bool {
    obj.values().any(|v| *v == Value::String("red".to_string()))
}
