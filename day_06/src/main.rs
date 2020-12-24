use std::fs;
use std::time::Instant;
use std::{collections::HashMap, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let questions_list = read_file("input.txt")?;

    let start = Instant::now();

    println!("Answer 1: {:?}", part_01(&questions_list));
    println!("Completed in {:?}", start.elapsed());

    let start = Instant::now();

    println!("Answer 2: {:?}", part_02(&questions_list));
    println!("Completed in {:?}", start.elapsed());

    Ok(())
}

fn part_01(questions_list: &Vec<String>) -> usize {
    questions_list
        .iter()
        .map(|group| {
            let mut characters: Vec<char> = group.replace(" ", "").chars().collect();
            characters.sort();
            characters.dedup();

            characters.iter().count()
        })
        .sum()
}

fn part_02(questions_list: &Vec<String>) -> usize {
    let mut count: usize = 0;
    let mut answers: Vec<HashMap<char, usize>> = Vec::new();

    for (index, group) in questions_list.iter().enumerate() {
        answers.push(HashMap::new());
        let group_size = group.split_whitespace().count();
        let persons = group.split_whitespace();

        for person in persons {
            for answer in person.chars() {
                *answers[index].entry(answer).or_insert(0) += 1
            }
        }

        count += answers[index]
            .iter()
            .filter(|(_, &answer_count)| answer_count == group_size)
            .count()
    }

    count
}

fn read_file(filename: &str) -> std::io::Result<Vec<String>> {
    let input = fs::read_to_string(filename)?;

    Ok(input
        .split("\r\n\r\n")
        .map(|line| line.to_string().replace("\r\n", " "))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_01() {
        let questions_list = read_file("example.txt").unwrap();

        assert_eq!(part_01(&questions_list), 11);
    }

    #[test]
    fn example_02() {
        let questions_list = read_file("example.txt").unwrap();

        assert_eq!(part_02(&questions_list), 6);
    }
}
