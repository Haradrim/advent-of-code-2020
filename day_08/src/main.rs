use std::{error::Error, str::FromStr};
use std::{fs, time::Instant};

fn main() -> Result<(), Box<dyn Error>> {
    let instructions = read_file("input.txt")?;

    let start = Instant::now();

    println!("Answer 1: {:?}", part_01(&instructions));
    println!("Completed in {:?}", start.elapsed());

    let start = Instant::now();

    println!("Answer 2: {:?}", part_02(&instructions));
    println!("Completed in {:?}", start.elapsed());

    Ok(())
}

fn part_01(instructions: &Vec<Instruction>) -> i32 {
    let mut console = Console {
        history: Vec::new(),
        instructions: instructions.clone(),
        accumulator: 0,
    };

    console.execute_instructions();

    console.accumulator
}

fn part_02(instructions: &Vec<Instruction>) -> i32 {
    let mut console = Console {
        history: Vec::new(),
        instructions: instructions.clone(),
        accumulator: 0,
    };

    console.execute_instructions();

    for index_history in console.history.iter() {
        let index = index_history.clone();
        let mut new_instructions: Vec<Instruction> = console.instructions.clone();
        let mut skip = false;

        new_instructions[index as usize] = match new_instructions[index as usize] {
            Instruction::ACC(arg) => { skip = true; Instruction::ACC(arg) },
            Instruction::JMP(arg) => Instruction::NOP(arg),
            Instruction::NOP(arg) => Instruction::JMP(arg),
        };

        if !skip {
            let mut new_console = Console {
                history: Vec::new(),
                instructions: new_instructions,
                accumulator: 0,
            };
    
            let has_loop = new_console.execute_instructions();
    
            if !has_loop {
                return new_console.accumulator;
            }
        }
    }

    panic!("Could not fix the program.")
}

fn read_file(filename: &str) -> std::io::Result<Vec<Instruction>> {
    let input = fs::read_to_string(filename)?;

    Ok(input.lines().filter_map(|line| line.parse().ok()).collect())
}

#[derive(Debug)]
struct Console {
    history: Vec<i32>,
    instructions: Vec<Instruction>,
    accumulator: i32,
}

impl Console {
    fn execute_instructions(&mut self) -> bool {
        let mut index: i32 = 0;
        let mut has_loop = false;

        while (index as usize) < self.instructions.len() {
            if self.history.contains(&index) {
                has_loop = true;
                break;
            }

            self.history.push(index);

            let (next_index, next_acc) =
                self.instructions[index as usize].execute(index, self.accumulator);

            index = next_index;
            self.accumulator = next_acc;
        }

        has_loop
    }
}
#[derive(Debug, Clone, Copy)]
enum Instruction {
    ACC(i32),
    JMP(i32),
    NOP(i32),
}

impl Instruction {
    fn execute(&self, index: i32, acc: i32) -> (i32, i32) {
        match self {
            Instruction::ACC(arg) => (index + 1, acc + arg),
            Instruction::JMP(arg) => (index + arg, acc),
            Instruction::NOP(_) => (index + 1, acc),
        }
    }
}

impl FromStr for Instruction {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(" ").collect();

        let instruction_string = split[0].to_uppercase();
        let argument: i32 = split[1].parse()?;

        Ok(match instruction_string.as_str() {
            "ACC" => Instruction::ACC(argument),
            "JMP" => Instruction::JMP(argument),
            "NOP" => Instruction::NOP(argument),
            _ => panic!("Invalid instruction"),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_01() {
        let instructions = read_file("example.txt").unwrap();

        assert_eq!(part_01(&instructions), 5);
    }

    #[test]
    fn example_02() {
        let instructions = read_file("example.txt").unwrap();

        assert_eq!(part_02(&instructions), 8);
    }
}
