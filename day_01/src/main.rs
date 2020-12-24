use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

const TARGET_VALUE: u32 = 2020;

fn main() -> Result<(), Box<dyn Error>> {
    // Read numbers input file
    let numbers = read_file("numbers.txt")?;

    let start = Instant::now();

    println!("Answer 1: {:?}", part_01(&numbers));
    println!("Completed in {:?}", start.elapsed());

    let start = Instant::now();

    println!("Answer 2: {:?}", part_02(&numbers));
    println!("Completed in {:?}", start.elapsed());

    /*
    // lot of people did it with itertools
    let result: u32 = numbers
        .combinations(3)
        .filter(|x| x[0] + x[1] + x[2] == TARGET_VALUE)
        .next()
        .unwrap()
        .iter()
        .product();
    */

    Ok(())
}

fn read_file(filename: &str) -> std::io::Result<Vec<u32>> {
    let input = File::open(filename)?;
    let reader = BufReader::new(input);

    Ok(reader
        .lines()
        .filter_map(|line| line.ok().and_then(|line| line.parse().ok()))
        .collect())
}

fn part_01(numbers: &Vec<u32>) -> Result<u32, &str> {
    for a in numbers.iter() {
        for b in numbers.iter() {
            if a + b == TARGET_VALUE {
                // return first match
                return Ok(a * b);
            }
        }
    }

    Err("Something went wrong")
}

fn part_02(numbers: &Vec<u32>) -> Result<u32, &str> {
    for a in numbers.iter() {
        for b in numbers.iter() {
            for c in numbers.iter() {
                if a + b + c == TARGET_VALUE {
                    // return first match
                    return Ok(a * b * c);
                }
            }
        }
    }

    Err("Something went wrong")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let numbers: Vec<u32> = vec![1721, 979, 366, 299, 675, 1456];

        assert_eq!(part_01(&numbers), Ok(514579));
    }
}
