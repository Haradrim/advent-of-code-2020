use std::error::Error;
use std::fs;
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    let boarding_passes = read_file("input.txt")?;

    let start = Instant::now();

    println!("Answer 1: {:?}", part_01(&boarding_passes));
    println!("Completed in {:?}", start.elapsed());

    println!("Answer 2: {:?}", part_02(&boarding_passes));
    println!("Completed in {:?}", start.elapsed());

    Ok(())
}

fn part_01(passes: &Vec<String>) -> u16 {
    passes
            .iter()
            .map(|x| calculate_seat(x))
            .max()
            .unwrap()
}

fn part_02(passes: &Vec<String>) -> u16 {
    let mut seats:Vec<u16> = passes
            .iter()
            .map(|x| calculate_seat(x))
            .collect();
            
    seats.sort_unstable();

    let result = (seats[0]..=seats[seats.len() - 1])
        .zip(seats
        .iter())
        .find(|(expected, seat)| expected != *seat)
        .unwrap();
    
    result.0
}

fn calculate_seat (boarding_pass: &String) -> u16
{
    boarding_pass.chars().fold(0,  |acc, character| {
        (acc << 1) | matches!(character, 'B' | 'R') as u16
    })
}

fn read_file(filename: &str) -> std::io::Result<Vec<String>> {
    let input = fs::read_to_string(filename)?;

    Ok(
        input
            .lines()
            .map(|line| line.to_string())
            .collect()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_01() {
        assert_eq!(calculate_seat(&"BFFFBBFRRR".to_string()), 567);
        assert_eq!(calculate_seat(&"FFFBBBFRRR".to_string()), 119);
        assert_eq!(calculate_seat(&"BBFFBBFRLL".to_string()), 820);
    }
}