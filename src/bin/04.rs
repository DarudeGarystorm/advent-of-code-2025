use advent_of_code::grid::{count_adjacent, parse_grid};

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse_grid(input);

    let mut count = 0;

    for (row, chars) in grid.iter().enumerate() {
        for (col, &ch) in chars.iter().enumerate() {
            if ch != '@' {
                continue;
            }

            if count_adjacent(&grid, row, col, '@') < 4 {
                count += 1;
            }
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid: Vec<Vec<char>> = parse_grid(input);

    let mut count = 0;

    let mut again = true;
    while again {
        again = false;

        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                let ch = grid[row][col];

                if ch != '@' {
                    continue;
                }

                if count_adjacent(&grid, row, col, '@') < 4 {
                    count += 1;
                    grid[row][col] = 'x';
                    again = true;
                }
            }
        }
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
