use std::{collections::HashMap, fs};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("../input.txt")?;

    // Part 1: Get the signal on wire 'a'
    let mut circuit = Circuit::new(&input);
    let part1 = circuit.get_signal("a");
    println!("Part 1: {}", part1);

    // Part 2: Override wire 'b' with the signal from wire 'a', reset other wires
    let mut circuit = Circuit::new(&input);
    circuit.override_wire("b", part1);
    let part2 = circuit.get_signal("a");
    println!("Part 2: {}", part2);

    Ok(())
}

// Circuit instructions
#[derive(Clone)]
enum Instruction {
    Value(u16),
    Wire(String),
    Not(Box<Source>),
    And(Box<Source>, Box<Source>),
    Or(Box<Source>, Box<Source>),
    LShift(Box<Source>, u16),
    RShift(Box<Source>, u16),
}

#[derive(Clone)]
enum Source {
    Literal(u16),
    Wire(String),
}

impl Source {
    fn parse(input: &str) -> Self {
        if let Ok(value) = input.parse::<u16>() {
            Source::Literal(value)
        } else {
            Source::Wire(input.to_string())
        }
    }

    fn evaluate(&self, circuit: &mut Circuit) -> u16 {
        match self {
            Source::Literal(value) => *value,
            Source::Wire(wire) => circuit.get_signal(wire),
        }
    }
}

struct Circuit {
    instructions: HashMap<String, Instruction>,
    cache: HashMap<String, u16>,
}

impl Circuit {
    fn new(input: &str) -> Self {
        let mut instructions = HashMap::new();

        for line in input.lines() {
            if let Some((expr, wire)) = line.split_once(" -> ") {
                let parts: Vec<&str> = expr.split_whitespace().collect();

                let instruction = match parts.len() {
                    1 => {
                        if let Ok(value) = parts[0].parse::<u16>() {
                            Instruction::Value(value)
                        } else {
                            Instruction::Wire(parts[0].to_string())
                        }
                    }
                    2 => {
                        // Only NOT operation has 2 parts
                        if parts[0] == "NOT" {
                            Instruction::Not(Box::new(Source::parse(parts[1])))
                        } else {
                            continue;
                        }
                    }
                    3 => {
                        let source1 = Source::parse(parts[0]);
                        let source2 = Source::parse(parts[2]);

                        match parts[1] {
                            "AND" => Instruction::And(Box::new(source1), Box::new(source2)),
                            "OR" => Instruction::Or(Box::new(source1), Box::new(source2)),
                            "LSHIFT" => {
                                if let Source::Literal(shift) = source2 {
                                    Instruction::LShift(Box::new(source1), shift)
                                } else {
                                    continue;
                                }
                            }
                            "RSHIFT" => {
                                if let Source::Literal(shift) = source2 {
                                    Instruction::RShift(Box::new(source1), shift)
                                } else {
                                    continue;
                                }
                            }
                            _ => continue,
                        }
                    }
                    _ => continue,
                };

                instructions.insert(wire.to_string(), instruction);
            }
        }

        Circuit {
            instructions,
            cache: HashMap::new(),
        }
    }

    fn override_wire(&mut self, wire: &str, value: u16) {
        self.instructions
            .insert(wire.to_string(), Instruction::Value(value));

        self.cache.clear();
    }

    fn get_signal(&mut self, wire: &str) -> u16 {
        if let Some(&signal) = self.cache.get(wire) {
            return signal;
        }

        let instruction = self.instructions.get(wire).cloned();
        let signal = match instruction {
            Some(Instruction::Value(value)) => value,
            Some(Instruction::Wire(source_wire)) => self.get_signal(&source_wire),
            Some(Instruction::Not(source)) => !source.evaluate(self),
            Some(Instruction::And(source1, source2)) => {
                source1.evaluate(self) & source2.evaluate(self)
            }
            Some(Instruction::Or(source1, source2)) => {
                source1.evaluate(self) | source2.evaluate(self)
            }
            Some(Instruction::LShift(source, shift)) => source.evaluate(self) << shift,
            Some(Instruction::RShift(source, shift)) => source.evaluate(self) >> shift,
            None => 0, // Wire not found
        };

        self.cache.insert(wire.to_string(), signal);
        signal
    }
}
