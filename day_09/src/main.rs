use std::{cmp::Ordering, error::Error, fs, time::Instant};

const PREAMBLE: usize = 25;

// TODO: solutions can probably be more optimized, they are a bit slow.
fn main() -> Result<(), Box<dyn Error>> {
    let numbers = read_file("input.txt")?;

    let start = Instant::now();

    let invalid_number = part_01(&numbers, PREAMBLE);

    println!("Answer 1: {:?}", invalid_number);
    println!("Completed in {:?}", start.elapsed());

    let start = Instant::now();

    println!("Answer 2: {:?}", part_02(&numbers, invalid_number.unwrap()));
    println!("Completed in {:?}", start.elapsed());

    Ok(())
}

fn part_01(numbers: &Vec<u64>, preamble: usize) -> Option<u64> {
    for (index, number) in numbers.iter().enumerate().skip(preamble) {
        let mut preamble: Vec<u64> = numbers[(index - preamble)..index].iter().copied().collect();

        preamble.sort();

        let is_valid = number_is_valid(&number, &preamble);

        if !is_valid {
            return Some(*number);
        }
    }

    None
}

fn part_02(numbers: &Vec<u64>, invalid_number: u64) -> Option<u64> {
    for (index, _) in numbers.iter().enumerate() {
        let range: Vec<u64> = numbers[index..].iter().copied().collect();
        let mut accumulator = 0;

        for (index, number) in range.iter().enumerate() {
            accumulator += number;

            match (accumulator).cmp(&invalid_number) {
                Ordering::Equal => {
                    let mut range: Vec<u64> = range[..=index].iter().copied().collect();

                    range.sort();

                    return Some(range[0] + range[range.len() - 1]);
                }
                Ordering::Greater => break,
                Ordering::Less => continue,
            }
        }
    }

    None
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

        assert_eq!(part_01(&numbers, 5), Some(127));
    }

    #[test]
    fn example_02() {
        let numbers = read_file("example.txt").unwrap();

        assert_eq!(part_02(&numbers, 127), Some(62));
    }
}
