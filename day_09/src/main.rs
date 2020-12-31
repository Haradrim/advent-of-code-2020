use std::{cmp::Ordering, error::Error, fs, time::Instant};

const PREAMBLE: usize = 25;

fn main() -> Result<(), Box<dyn Error>> {
    let numbers = read_file("input.txt")?;

    let start = Instant::now();

    println!("Answer 1: {:?}", part_01(&numbers, PREAMBLE));
    println!("Completed in {:?}", start.elapsed());

    Ok(())
}

fn part_01(numbers: &Vec<u64>, preamble: usize) -> u64 {
    for (index, number) in numbers.iter().enumerate().skip(preamble) {
        let mut preamble: Vec<u64> = numbers[(index - preamble)..index].iter().copied().collect();

        preamble.sort();

        let is_valid = number_is_valid(&number, &preamble);

        if !is_valid {
            return *number;
        }
    }

    panic!("No number found.")
}

fn number_is_valid(number: &u64, preamble: &Vec<u64>) -> bool {
    for i in preamble {
        for j in preamble {
            match (i + j).cmp(number) {
                Ordering::Equal => return true,
                Ordering::Greater => break,
                Ordering::Less => continue,
            }
        }
    }

    return false;
}

fn read_file(filename: &str) -> std::io::Result<Vec<u64>> {
    let input = fs::read_to_string(filename)?;

    Ok(input.lines().filter_map(|line| line.parse().ok()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_01() {
        let numbers = read_file("example.txt").unwrap();

        assert_eq!(part_01(&numbers, 5), 127);
    }
}
