use serde_json::{Value, json};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let input = fs::read_to_string("../input.json")?;

    let json_value: Value = serde_json::from_str(&input)?;

    let total_sum = sum_numbers(&json_value);

    println!("Total sum: {}", total_sum);
    Ok(())
}

fn sum_numbers(value: &Value) -> i64 {
    match value {
        Value::Number(num) => {
            if let Some(n) = num.as_i64() {
                return n;
            }
            0
        }
        Value::Array(arr) => arr.iter().map(|v| sum_numbers(v)).sum(),
        Value::Object(obj) => {
            // Check if the object contains "red"
            if obj.values().any(|v| v == &Value::String("red".to_string())) {
                return 0;
            }

            obj.values().map(|v| sum_numbers(v)).sum()
        }
        _ => 0,
    }
}

