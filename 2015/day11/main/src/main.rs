fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input: &str = "vzbxxyzz";

    let mut password = input.chars().collect::<Vec<char>>();

    loop {
        increment_password(&mut password);

        let is_valid = verify_password(&password);

        if is_valid {
            break;
        }
    }

    println!("Current password: {}", password.iter().collect::<String>());

    Ok(())
}

fn verify_password(password: &Vec<char>) -> bool {
    if password.contains(&'i') || password.contains(&'o') || password.contains(&'l') {
        return false;
    }

    let mut pairs: usize = 0;
    let mut i = 0;
    while i < password.len() - 1 {
        if password[i] == password[i + 1] {
            pairs += 1;
            i += 2;
        } else {
            i += 1;
        }
    }

    if pairs < 2 {
        return false;
    }

    if !password.windows(3).any(|w| {
        let first = w[0] as u8;
        let second = w[1] as u8;
        let third = w[2] as u8;

        first + 1 == second && second + 1 == third
    }) {
        return false;
    }

    true
}

fn increment_password(password: &mut Vec<char>) {
    for i in (0..password.len()).rev() {
        if password[i] == 'z' {
            password[i] = 'a';
        } else {
            password[i] = (password[i] as u8 + 1) as char;
            break;
        }
    }
}
