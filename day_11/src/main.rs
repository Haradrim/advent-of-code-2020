use std::{error::Error, fs, time::Instant};

fn main() -> Result<(), Box<dyn Error>> {
    let rows = read_file("input.txt")?;

    let start = Instant::now();

    println!("Answer 1: {:?}", part_01(&rows));
    println!("Completed in {:?}", start.elapsed());

    Ok(())
}

fn count_occupied_neighbors(grid: &Vec<Vec<char>>, row_index: usize, col_index: usize) -> u64 {
    let movements = [
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, 1),
        (1, 1),
        (-1, -1),
        (1, -1),
    ];

    let mut count = 0;
    for (x, y) in &movements {
        let x_index = row_index as isize + x;
        let y_index = col_index as isize + y;

        // check if index is off grid
        if x_index >= 0
            && x_index < (grid.len() as isize)
            && y_index >= 0
            && y_index < (grid[x_index as usize].len() as isize)
        {
            match grid[x_index as usize][y_index as usize] {
                '#' => count += 1,
                _ => (),
            }
        }
    }

    count
}

fn part_01(rows: &Vec<Vec<char>>) -> usize {
    let mut current = rows.clone();
    let mut next: Vec<Vec<char>>;

    loop {
        next = generate_next(&current);

        if next == current {
            return current
                .iter()
                .flatten()
                .filter(|seat| **seat == '#')
                .count();
        }

        current = next;
    }
}

fn generate_next(rows: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut next = rows.clone();

    for (row_index, row) in rows.iter().enumerate() {
        for (seat_index, seat) in row.iter().enumerate() {
            let new_state = match *seat {
                'L' if count_occupied_neighbors(&rows, row_index, seat_index) == 0 => '#',
                '#' if count_occupied_neighbors(&rows, row_index, seat_index) >= 4 => 'L',
                c => c,
            };

            next[row_index][seat_index] = new_state;
        }
    }

    next
}

fn read_file(filename: &str) -> std::io::Result<Vec<Vec<char>>> {
    let input = fs::read_to_string(filename)?;

    Ok(input.lines().map(|line| line.chars().collect()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_01() {
        let rows = read_file("example.txt").unwrap();

        assert_eq!(part_01(&rows), 37);
    }
}
