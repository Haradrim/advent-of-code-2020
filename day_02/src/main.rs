use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::num::ParseIntError;
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    let rules = read_file("input.txt")?;

    let start = Instant::now();
    
    println!("Answer 1: {:?}", part_01(&rules));
    println!("Completed in {:?}", start.elapsed());

    let start = Instant::now();

    println!("Answer 2: {:?}", part_02(&rules));
    println!("Completed in {:?}", start.elapsed());

    Ok(())
}

fn part_01 (rules: &Vec<Rule>) -> usize {
    rules.iter().filter(|rule| rule.is_valid_01()).count()
}

fn part_02 (rules: &Vec<Rule>) -> usize {
    rules.iter().filter(|rule| rule.is_valid_02()).count()
} 

#[derive(Debug)]
struct Rule {
    min_char: usize,
    max_char: usize,
    required_char: char,
    password: String
}

impl Rule {
    fn is_valid_01(&self) -> bool {
        let required_char_count = self.password.chars().filter(|&c| c == self.required_char).count();

        required_char_count >= self.min_char && required_char_count <= self.max_char
    }

    fn is_valid_02(&self) -> bool {
        let password_chars: Vec<char> = self.password.chars().collect();
        let index_01 = self.min_char - 1;
        let index_02 = self.max_char - 1;

        (password_chars[index_01] == self.required_char && password_chars[index_02] != self.required_char) || (password_chars[index_01] != self.required_char && password_chars[index_02] == self.required_char)
    }
}

impl FromStr for Rule {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sub_strings: Vec<&str> = s.trim().split(':').collect();
        let (prefix, password) = (sub_strings[0], sub_strings[1]);
        let password: String = password.trim().to_string();

        let sub_strings: Vec<&str> = prefix.trim().split(' ').collect();
        let (bounds, character) = (sub_strings[0], sub_strings[1]);

        let required_char: char = character.trim().chars().collect::<Vec<char>>()[0];

        let sub_strings: Vec<&str> = bounds.trim().split('-').collect();
        let (min, max) = (sub_strings[0], sub_strings[1]);
         
        let min_char: usize = min.parse()?;
        let max_char: usize = max.parse()?;
        

        Ok(Rule {
            min_char,
            max_char,
            required_char,
            password
        })
    }
}

fn read_file(filename: &str) -> std::io::Result<Vec<Rule>> {
    let input = File::open(filename)?;
    let reader = BufReader::new(input);

    Ok(
        reader
            .lines()
            .filter_map(|line| line.ok().and_then(|line| line.parse().ok()))
            .collect()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_01() {
        let rule1 = Rule { 
            min_char: 1,
            max_char: 3,
            required_char: 'a',
            password: "abcde".to_string()
        };

        let rule2 = Rule { 
            min_char: 1,
            max_char: 3,
            required_char: 'b',
            password: "cdefg".to_string()
        };

        let rule3 = Rule { 
            min_char: 2,
            max_char: 9,
            required_char: 'c',
            password: "ccccccccc".to_string()
        };

        let rules: Vec<Rule> = vec![rule1, rule2, rule3];
        
        assert_eq!(part_01(&rules), 2);
    }

    #[test]
    fn examples() {
        let rule1 = Rule { 
            min_char: 1,
            max_char: 3,
            required_char: 'a',
            password: "abcde".to_string()
        };

        let rule2 = Rule { 
            min_char: 1,
            max_char: 3,
            required_char: 'b',
            password: "cdefg".to_string()
        };

        let rule3 = Rule { 
            min_char: 2,
            max_char: 9,
            required_char: 'c',
            password: "ccccccccc".to_string()
        };

        let rules: Vec<Rule> = vec![rule1, rule2, rule3];
        
        assert_eq!(part_01(&rules), 2);

        assert_eq!(part_02(&rules), 1);
    }
}