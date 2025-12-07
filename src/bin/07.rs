use advent_of_code::grid::parse_grid;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let mut grid = parse_grid(input);

    let rows = grid.len();
    let cols = grid[0].len();

    let mut count = 0;

    for row in 0..rows {
        for col in 0..cols {
            let ch = grid[row][col];

            if ch == 'S' || ch == '|' {
                // can't go down if we're the last row
                if row == rows - 1 {
                    continue;
                }

                if grid[row + 1][col] == '^' {
                    count += 1;

                    // Add | to the left if not at left boundary
                    if col > 0 {
                        grid[row + 1][col - 1] = '|';
                    }

                    // Add | to the right if not at right boundary
                    if col < cols - 1 {
                        grid[row + 1][col + 1] = '|';
                    }
                } else {
                    grid[row + 1][col] = '|';
                }
            }
        }
    }

    Some(count)
}

#[derive(Debug, Clone, Copy)]
struct Cell {
    ch: char,
    count: u64,
}

pub fn part_two(input: &str) -> Option<u64> {
    let char_grid = parse_grid(input);

    let rows = char_grid.len();
    let cols = char_grid[0].len();

    let mut grid: Vec<Vec<Cell>> = char_grid
        .iter()
        .map(|row| {
            row.iter()
                .map(|&ch| Cell {
                    ch,
                    count: if ch == 'S' { 1 } else { 0 },
                })
                .collect()
        })
        .collect();

    for row in 0..rows - 1 {
        for col in 0..cols {
            let Cell { ch, count } = grid[row][col];

            if ch == 'S' || ch == '|' {
                if grid[row + 1][col].ch == '^' {
                    // Add | to the left if not at left boundary
                    if col > 0 {
                        let left_cell = &mut grid[row + 1][col - 1];
                        left_cell.ch = '|';
                        left_cell.count += count;
                    }

                    // Add | to the right if not at right boundary
                    if col < cols - 1 {
                        let right_cell = &mut grid[row + 1][col + 1];
                        right_cell.ch = '|';
                        right_cell.count += count;
                    }
                } else {
                    let below_cell = &mut grid[row + 1][col];
                    below_cell.ch = '|';
                    below_cell.count += count;
                }
            }
        }
    }

    Some(grid.last()?.iter().fold(0, |acc, cell| acc + cell.count))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
