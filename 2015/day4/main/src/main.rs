use md5;
fn main() {
    let input: &str = "ckczppom";

    let hex_start: &str = "000000";

    let mut number: u32 = 0;

    loop {
        let hash = md5::compute(format!("{}{}", input, number));
        let hash_str = format!("{:x}", hash);
        if hash_str.starts_with(hex_start) {
            println!("Number: {}", number);
            break;
        }
        number = increment_number(number);
    }
}

fn increment_number(start_number: u32) -> u32 {
    start_number + 1
}
