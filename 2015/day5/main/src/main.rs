use std::fs;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("../input.txt")?;
    let mut nice_strings = 0;
    for line in input.lines() {
        if verify_nice_string_2(line) {
            nice_strings += 1;
        }
    }
    println!("Number of nice strings: {}", nice_strings);
    Ok(())
}

fn verify_nice_string_2(input_string: &str) -> bool {
    // Check for pairs that appear twice without overlapping
    let mut has_repeated_pair = false;
    for i in 0..input_string.len() - 1 {
        let pair = &input_string[i..i + 2];

        let pair_count = input_string.matches(pair).count();
        if pair_count >= 2 {
            has_repeated_pair = true;
            break;
        }
    }

    // Check for repeated letter with exactly one letter between
    let mut has_repeated_with_one_between = false;
    for i in 0..input_string.len() - 2 {
        if input_string.chars().nth(i) == input_string.chars().nth(i + 2) {
            has_repeated_with_one_between = true;
            break;
        }
    }

    has_repeated_pair && has_repeated_with_one_between
}

fn verify_nice_string(input_string: &str) -> bool {
    // nice string: at least 3 vowels (aeiou), double letter (aa, bb, etc), no bad strings (ab, cd,
    // pq, or xy)
    let vowels: Vec<char> = vec!['a', 'e', 'i', 'o', 'u'];
    let bad_strings: Vec<&str> = vec!["ab", "cd", "pq", "xy"];
    let mut vowel_count = 0;
    let mut double_letter = false;
    let mut bad_string = false;

    for (i, c) in input_string.chars().enumerate() {
        if vowels.contains(&c) {
            vowel_count += 1;
        }
        if i > 0 {
            let prev_char = input_string.chars().nth(i - 1).unwrap();
            if prev_char == c {
                double_letter = true;
            }
            let prev_chars = format!("{}{}", prev_char, c);
            if bad_strings.contains(&prev_chars.as_str()) {
                bad_string = true;
            }
        }
    }

    if vowel_count >= 3 && double_letter && !bad_string {
        return true;
    }

    false
}
