use std::{error::Error, fs, time::Instant};

fn main() -> Result<(), Box<dyn Error>> {
    let rows = read_file("input.txt")?;

    let start = Instant::now();

    println!("Answer 1: {:?}", part_01(&rows));
    println!("Completed in {:?}", start.elapsed());

    let start = Instant::now();

    println!("Answer 2: {:?}", part_02(&rows));
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
    for (row, col) in &movements {
        let row_index = row_index as isize + row;
        let col_index = col_index as isize + col;

        // check if index is off grid
        if is_on_grid(&grid, row_index, col_index) {
            match grid[row_index as usize][col_index as usize] {
                '#' => count += 1,
                _ => (),
            }
        }
    }

    count
}

fn count_visible_occupied_neighbors(
    grid: &Vec<Vec<char>>,
    row_index: usize,
    col_index: usize,
) -> u64 {
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
    for (row, col) in &movements {
        let mut n = 1;

        loop {
            let row_index = row_index as isize + row * n;
            let col_index = col_index as isize + col * n;

            // check if index is off grid
            if is_on_grid(&grid, row_index, col_index) {
                match grid[row_index as usize][col_index as usize] {
                    '#' => {
                        count += 1;
                        break;
                    }
                    'L' => break,
                    _ => (),
                }

                n += 1;
            } else {
                break;
            }
        }
    }

    count
}

fn is_on_grid(grid: &Vec<Vec<char>>, row_index: isize, col_index: isize) -> bool {
    if row_index >= 0
        && row_index < (grid.len() as isize)
        && col_index >= 0
        && col_index < (grid[row_index as usize].len() as isize)
    {
        return true;
    }

    false
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

fn part_02(rows: &Vec<Vec<char>>) -> usize {
    let mut current = rows.clone();
    let mut next: Vec<Vec<char>>;

    loop {
        next = generate_next_visible(&current);

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

fn generate_next_visible(rows: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut next = rows.clone();

    for (row_index, row) in rows.iter().enumerate() {
        for (seat_index, seat) in row.iter().enumerate() {
            let new_state = match *seat {
                'L' if count_visible_occupied_neighbors(&rows, row_index, seat_index) == 0 => '#',
                '#' if count_visible_occupied_neighbors(&rows, row_index, seat_index) >= 5 => 'L',
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

    #[test]
    fn example_02() {
        let rows = read_file("example.txt").unwrap();

        assert_eq!(part_02(&rows), 26);
    }
}
