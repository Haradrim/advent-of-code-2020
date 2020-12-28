use std::{error::Error, str::FromStr};
use std::{fs, time::Instant};

fn main() -> Result<(), Box<dyn Error>> {
    let instructions = read_file("input.txt")?;

    let start = Instant::now();

    println!("Answer 1: {:?}", part_01(&instructions));
    println!("Completed in {:?}", start.elapsed());

    Ok(())
}

fn part_01(instructions: &Vec<Instruction>) -> i32 {
    let mut console = Console {
        instructions: instructions.clone(),
        accumulator: 0,
    };

    console.execute_instructions();

    console.accumulator
}

fn read_file(filename: &str) -> std::io::Result<Vec<Instruction>> {
    let input = fs::read_to_string(filename)?;

    Ok(input.lines().filter_map(|line| line.parse().ok()).collect())
}

#[derive(Debug)]
struct Console {
    instructions: Vec<Instruction>,
    accumulator: i32,
}

impl Console {
    fn execute_instructions(&mut self) {
        let mut index: i32 = 0;
        let mut history: Vec<i32> = Vec::new();

        while (index as usize) < self.instructions.len() {
            if history.contains(&index) {
                break;
            }

            history.push(index);

            let (next_index, next_acc) =
                self.instructions[index as usize].execute(index, self.accumulator);

            index = next_index;
            self.accumulator = next_acc;
        }
    }
}
#[derive(Debug, Clone, Copy)]
enum Instruction {
    ACC(i32),
    JMP(i32),
    NOP,
}

impl Instruction {
    fn execute(&self, index: i32, acc: i32) -> (i32, i32) {
        match self {
            Instruction::ACC(arg) => (index + 1, acc + arg),
            Instruction::JMP(arg) => (index + arg, acc),
            Instruction::NOP => (index + 1, acc),
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
            "NOP" => Instruction::NOP,
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
}
