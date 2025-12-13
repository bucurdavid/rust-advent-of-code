use std::fs;
use std::str::FromStr;

#[derive(Debug)]
pub enum KnobDirection {
    Left,
    Right,
}

impl FromStr for KnobDirection {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(KnobDirection::Left),
            "R" => Ok(KnobDirection::Right),
            _ => Err(format!("Invalid direction: {}", s)),
        }
    }
}

#[derive(Debug)]
pub struct Command {
    pub direction: KnobDirection,
    pub value: i32,
}

#[derive(Debug, Clone)]
pub struct Dial {
    pub position: i32,
    pub range: i32,
}

impl Dial {
    pub fn turn(&mut self, command: &Command) -> i32 {
        match command.direction {
            KnobDirection::Left => {
                let new_position = self.position - command.value;
                self.position = (self.position - command.value).rem_euclid(self.range);
                new_position
            }
            KnobDirection::Right => {
                let new_position = self.position + command.value;
                self.position = (self.position + command.value).rem_euclid(self.range);
                new_position
            }
        }
    }

    pub fn count_zero_hits(&self, command: &Command) -> u32 {
        let range = self.range;
        let pos = self.position.rem_euclid(range);
        let value = command.value;

        if value <= 0 {
            return 0;
        }

        let first_hit = match command.direction {
            KnobDirection::Right => {
                if pos == 0 {
                    range
                } else {
                    range - pos
                }
            }
            KnobDirection::Left => {
                if pos == 0 {
                    range
                } else {
                    pos
                }
            }
        };

        if value < first_hit {
            0
        } else {
            ((value - first_hit) / range + 1) as u32
        }
    }
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err("Empty string".to_string());
        }

        let direction: KnobDirection = s[0..1].parse()?;
        let value: i32 = s[1..]
            .parse()
            .map_err(|e| format!("Invalid number: {}", e))?;

        Ok(Command { direction, value })
    }
}

fn puzzle1(input: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut dial_password_counter = 0u32;
    let mut dial = Dial {
        position: 50,
        range: 100,
    };

    for line in input.lines() {
        let cmd: Command = line.trim().parse()?;
        dial.turn(&cmd);
        if dial.position == 0 {
            dial_password_counter += 1;
        }
    }

    println!("Final Dial Position: {}", dial.position);
    println!("Dial Password Counter: {}", dial_password_counter);

    Ok(())
}

fn puzzle2(input: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut dial_password_counter = 0u32;
    let mut dial = Dial {
        position: 50,
        range: 100,
    };

    for line in input.lines() {
        let cmd: Command = line.trim().parse()?;

        // Count zero hits during this command
        dial_password_counter += dial.count_zero_hits(&cmd);

        // Apply the turn (dial position after rotation is irrelevant for counting hits mid-rotation)
        dial.turn(&cmd);
    }

    println!(
        "Dial Password Counter (method 0x434C49434B): {}",
        dial_password_counter
    );
    Ok(())
}

fn test_example() -> Result<(), Box<dyn std::error::Error>> {
    let example_input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
    
    println!("=== Testing Example Case ===");
    println!("Expected Puzzle 1 result: 3");
    println!("Expected Puzzle 2 result: 6");
    
    // Test Puzzle 2 with detailed debug
    let mut dial = Dial {
        position: 50,
        range: 100,
    };
    let mut puzzle2_count = 0u32;

    for (i, line) in example_input.lines().enumerate() {
        let cmd: Command = line.trim().parse()?;
        let hits = dial.count_zero_hits(&cmd);
        puzzle2_count += hits;
        dial.turn(&cmd);
        println!(
            "Step {}: {:?}, hits={}, total={}",
            i + 1,
            cmd,
            hits,
            puzzle2_count
        );
    }

    println!("Example Puzzle 2 result: {}", puzzle2_count);
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_bytes: Vec<u8> = fs::read("../inputs/puzzle1.txt")?;
    let input = String::from_utf8(input_bytes)?;

    let _ = puzzle1(input.clone());
    let _ = puzzle2(input.clone());

    Ok(())
}
