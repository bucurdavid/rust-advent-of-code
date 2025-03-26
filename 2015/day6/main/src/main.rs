use std::fs;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let inputs = fs::read_to_string("../input.txt")?;
    let mut light_grid = LightGrid::new();
    for line in inputs.lines() {
        let splitted_input = line.split_whitespace().collect::<Vec<&str>>();
        let instruction = splitted_input[..splitted_input.len() - 3].join(" ");
        let start = splitted_input[splitted_input.len() - 3]
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let end = splitted_input[splitted_input.len() - 1]
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        light_grid.apply_instruction(instruction.as_str(), (start[0], start[1]), (end[0], end[1]));
    }
    println!("Lights on: {}", light_grid.count_lights_brightness());
    Ok(())
}

enum Instruction {
    TurnOn,
    TurnOff,
    Toggle,
}

impl Instruction {
    fn from_str(s: &str) -> Self {
        match s.trim().to_lowercase().as_str() {
            "turn on" => Instruction::TurnOn,
            "turn off" => Instruction::TurnOff,
            "toggle" => Instruction::Toggle,
            _ => panic!("Invalid instruction"),
        }
    }
}

pub struct LightGrid {
    lights: Vec<Vec<u8>>,
}

impl LightGrid {
    pub fn new() -> Self {
        LightGrid {
            lights: vec![vec![0; 1000]; 1000],
        }
    }

    pub fn apply_instruction(
        &mut self,
        instruction: &str,
        start: (usize, usize),
        end: (usize, usize),
    ) {
        for ix in start.0..=end.0 {
            for iy in start.1..=end.1 {
                let instr = Instruction::from_str(instruction);

                match instr {
                    Instruction::TurnOn => self.lights[ix][iy] += 1,
                    Instruction::TurnOff => {
                        if self.lights[ix][iy] > 0 {
                            self.lights[ix][iy] -= 1;
                        }
                    }
                    Instruction::Toggle => self.lights[ix][iy] += 2,
                }
            }
        }
    }
    pub fn count_lights_brightness(&self) -> usize {
        self.lights.iter().flatten().map(|&x| x as usize).sum()
    }
}
