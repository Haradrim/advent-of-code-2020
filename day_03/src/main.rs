use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

const INIT_POSITION: usize = 0;
const TREE: char = '#';

fn main() -> Result<(), Box<dyn Error>> {
    let map = read_file("map.txt")?;

    let start = Instant::now();

    let tree_count = part_01(&map);

    println!("Answer 1: {:?}", tree_count);
    println!("Completed in {:?}", start.elapsed());

    let result = part_02(&map);

    println!("Answer 2: {:?}", result);
    println!("Completed in {:?}", start.elapsed());

    Ok(())
}

fn part_01(map: &Vec<Vec<char>>) -> u32 {
    traverse_map(map, (3, 1))
}

fn part_02(map: &Vec<Vec<char>>) -> u32 {
    let slopes: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut result = 1;

    for slope in slopes.iter() {
        result *= traverse_map(map, *slope)
    }

    result
}

fn traverse_map(map: &Vec<Vec<char>>, (right, down): (usize, usize)) -> u32 {
    let mut position = INIT_POSITION;
    let mut tree_count = 0;

    for line in map.iter().step_by(down) {
        if line[position] == TREE {
            tree_count += 1
        }

        position += right;

        if position >= line.len() {
            position -= line.len();
        }
    }

    tree_count
}

fn read_file(filename: &str) -> std::io::Result<Vec<Vec<char>>> {
    let input = File::open(filename)?;
    let reader = BufReader::new(input);

    Ok(reader
        .lines()
        .filter_map(|line| line.ok().and_then(|line| line.parse::<String>().ok()))
        .map(|map_line| map_line.chars().collect())
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_01() {
        let map = read_file("example.txt").unwrap();

        assert_eq!(part_01(&map), 7);
    }

    #[test]
    fn example_02() {
        let map = read_file("example.txt").unwrap();

        assert_eq!(part_02(&map), 336);
    }
}
