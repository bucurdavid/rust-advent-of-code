use std::fs;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("../input.txt")?;
    let mut total_string_literal_chars = 0;
    let mut total_in_memory_chars = 0;
    let mut total_encoded_chars = 0;
    for line in input.lines() {
        total_string_literal_chars += line.len() as u32;
        total_in_memory_chars += get_in_memory_chars(line);
        total_encoded_chars += get_encoded_chars(line);
    }

    println!(
        "difference: {}",
        total_string_literal_chars - total_in_memory_chars
    );
    println!(
        "encoded difference: {}",
        total_encoded_chars - total_string_literal_chars
    );
    Ok(())
}

fn get_in_memory_chars(s: &str) -> u32 {
    let s = &s[1..s.len() - 1];
    let mut chars = s.chars().peekable();
    let mut count = 0;

    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('\\') | Some('"') => {
                    count += 1;
                }
                Some('x') => {
                    if chars.next().is_some() && chars.next().is_some() {
                        count += 1;
                    } else {
                        count += 1;
                    }
                }
                Some(_) | None => {
                    count += 1;
                }
            }
        } else {
            count += 1;
        }
    }

    count
}
fn get_encoded_chars(s: &str) -> u32 {
    let mut encoded_length: u32 = 2;

    for c in s.chars() {
        match c {
            '\\' | '"' => encoded_length += 2,
            _ => encoded_length += 1,
        }
    }

    encoded_length
}
