use std::error::Error;
use std::fs;
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    let boarding_passes = read_file("input.txt")?;

    let start = Instant::now();

    println!("Answer 1: {:?}", part_01(&boarding_passes));
    println!("Completed in {:?}", start.elapsed());

    Ok(())
}

fn part_01(seats: &Vec<String>) -> u16 {
    seats
            .iter()
            .map(|x| calculate_seat(x))
            .max()
            .unwrap()
}

fn part_02(seats: &Vec<String>) {

}

fn calculate_seat (boarding_pass: &String) -> u16
{
    boarding_pass.chars().fold(0,  |acc, character| {
        (acc << 1) | if character == 'B' || character == 'R' { 1 } else { 0 }
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