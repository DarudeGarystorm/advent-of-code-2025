advent_of_code::solution!(6);

use advent_of_code::grid;

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.trim().lines();

    let operators: Vec<&str> = lines.next_back()?.split_whitespace().collect();

    let sum = lines
        .map(|line| {
            line.split_whitespace()
                .filter_map(|num| num.parse::<u64>().ok())
                .collect::<Vec<u64>>()
        })
        .fold(vec![0u64; operators.len()], |mut acc, nums| {
            for (i, op) in operators.iter().enumerate() {
                if acc[i] == 0 {
                    acc[i] = nums[i];
                } else if op == &"+" {
                    acc[i] += nums[i];
                } else if op == &"*" {
                    acc[i] *= nums[i];
                }
            }

            acc
        })
        .into_iter()
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = grid::parse_grid(input);

    let rows = grid.len();
    let cols = grid.iter().map(|row| row.len()).max().unwrap_or(0);

    let mut total_total: u64 = 0;
    let mut problem_total: u64 = 0;
    let mut problem_operator: char = ' ';

    for col in 0..cols {
        let mut num = String::new();

        for row in grid.iter().take(rows - 1) {
            if let Some(&ch) = row.get(col) {
                num.push(ch);
            }
        }

        if let Some(&ch @ ('+' | '*')) = grid.get(rows - 1).and_then(|row| row.get(col)) {
            problem_operator = ch;
        }

        if !num.trim().is_empty() {
            let parsed = num.trim().parse::<u64>().ok()?;

            if problem_total == 0 {
                problem_total = parsed;
            } else if problem_operator == '+' {
                problem_total += parsed;
            } else if problem_operator == '*' {
                problem_total *= parsed;
            }
        } else {
            total_total += problem_total;
            problem_total = 0;
            continue;
        }
    }

    if problem_total != 0 {
        total_total += problem_total;
    }

    Some(total_total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
