use std::fs;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_butes: Vec<u8> = fs::read("../inputs/puzzle1.txt")?;

    // puzzle 1
    let mut floor = 0;
    for (index, byte) in input_butes.iter().enumerate() {
        match byte {
            0x28 => floor += 1,
            0x29 => floor -= 1,
            _ => (),
        }
        //puzzle 2
        if floor == -1 {
            println!("Entered basement at position: {}", index + 1);
        }
    }
    println!("Floor: {}", floor);

    Ok(())
}
