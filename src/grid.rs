pub fn parse_grid(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

pub fn count_adjacent(grid: &[Vec<char>], row: usize, col: usize, target: char) -> u64 {
    let mut count = 0;
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for (dr, dc) in directions.iter() {
        let new_row = row as i32 + dr;
        let new_col = col as i32 + dc;

        if new_row >= 0
            && new_row < rows
            && new_col >= 0
            && new_col < cols
            && grid[new_row as usize][new_col as usize] == target
        {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_adjacent_basic() {
        let grid = vec![
            vec!['.', '#', '.'],
            vec!['#', '.', '#'],
            vec!['.', '#', '.'],
        ];
        assert_eq!(count_adjacent(&grid, 1, 1, '#'), 4);
        assert_eq!(count_adjacent(&grid, 0, 0, '#'), 2);
        assert_eq!(count_adjacent(&grid, 2, 2, '#'), 2);
    }

    #[test]
    fn test_count_adjacent_edges_and_corners() {
        let grid = vec![
            vec!['#', '.', '.'],
            vec!['.', '#', '.'],
            vec!['.', '.', '#'],
        ];
        // Top-left corner
        assert_eq!(count_adjacent(&grid, 0, 0, '#'), 1);
        // Top-right corner
        assert_eq!(count_adjacent(&grid, 0, 2, '#'), 1);
        // Bottom-left corner
        assert_eq!(count_adjacent(&grid, 2, 0, '#'), 1);
        // Bottom-right corner
        assert_eq!(count_adjacent(&grid, 2, 2, '#'), 1);
    }

    #[test]
    fn test_count_adjacent_no_matches() {
        let grid = vec![
            vec!['.', '.', '.'],
            vec!['.', '.', '.'],
            vec!['.', '.', '.'],
        ];
        assert_eq!(count_adjacent(&grid, 1, 1, '#'), 0);
    }

    #[test]
    fn test_count_adjacent_all_matches() {
        let grid = vec![
            vec!['#', '#', '#'],
            vec!['#', '#', '#'],
            vec!['#', '#', '#'],
        ];
        assert_eq!(count_adjacent(&grid, 1, 1, '#'), 8);
    }
}
