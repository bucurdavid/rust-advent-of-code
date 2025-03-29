use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::from("1113222113");
    let repeat = 50;

    for _ in 0..repeat {
        input = look_and_say(&input);
    }

    println!("Length of the final string: {}", input.len());
    Ok(())
}

fn look_and_say(previous: &str) -> String {
    let mut result = String::new();

    let digits = previous.chars().collect::<Vec<_>>();

    let mut i = 0;
    while i < digits.len() {
        let current_digit = digits[i];
        let mut count = 1;

        while i + 1 < digits.len() && digits[i + 1] == current_digit {
            count += 1;
            i += 1;
        }

        result.push_str(&count.to_string());
        result.push(current_digit);

        i += 1;
    }

    result
}
