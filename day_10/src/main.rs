use std::{error::Error, fs, time::Instant};

fn main() -> Result<(), Box<dyn Error>> {
    let voltage_ratings = read_file("input.txt")?;

    let start = Instant::now();

    println!("Answer 1: {:?}", part_01(&voltage_ratings));
    println!("Completed in {:?}", start.elapsed());

    let start = Instant::now();

    println!("Answer 2: {:?}", part_02(&voltage_ratings));
    println!("Completed in {:?}", start.elapsed());

    Ok(())
}

fn part_01(voltage_ratings: &Vec<u64>) -> u64 {
    let mut ratings: Vec<u64> = voltage_ratings.iter().copied().collect();
    let mut count_1: u64 = 0;
    let mut count_3: u64 = 0;

    ratings.push(0);
    ratings.sort();
    ratings.push(ratings[ratings.len() - 1] + 3);

    let mut index = 0;
    while index < ratings.len() - 1 {
        match ratings[index + 1] - ratings[index] {
            1 => count_1 += 1,
            3 => count_3 += 1,
            _ => (),
        }

        index += 1;
    }

    count_1 * count_3
}

fn part_02(voltage_ratings: &Vec<u64>) -> Option<u64> {
    let mut ratings: Vec<u64> = voltage_ratings.iter().copied().collect();

    ratings.push(0);
    ratings.sort();

    let mut methods: Vec<u64> = vec![0u64; ratings.len()];
    methods[0] = 1;

    for (start_index, start_rating) in ratings.iter().enumerate() {
        let value = methods[start_index];

        for (index, rating) in ratings[start_index..].iter().enumerate().skip(1) {
            if rating - start_rating <= 3 {
                methods[start_index + index] += value;
            } else {
                break;
            }
        }
    }

    methods.last().copied()
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
        let ratings = read_file("example.txt").unwrap();

        assert_eq!(part_01(&ratings), 220);
    }

    #[test]
    fn example_02() {
        let ratings = read_file("example.txt").unwrap();

        assert_eq!(part_02(&ratings), Some(19208));
    }
}
