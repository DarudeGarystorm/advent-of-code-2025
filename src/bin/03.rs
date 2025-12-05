advent_of_code::solution!(3);

fn find_max_joltage(line: &str, num_batteries: usize) -> u64 {
    let chars: Vec<char> = line.chars().collect();
    let n = chars.len();

    let mut result = String::with_capacity(num_batteries);
    let mut start_idx = 0;

    for pos in 0..num_batteries {
        let remaining_needed = num_batteries - pos - 1;
        let mut best_idx = start_idx;
        let mut best_val = chars[start_idx];

        // Look for the best digit we can pick at this position
        for i in start_idx..(n - remaining_needed) {
            if chars[i] > best_val {
                best_idx = i;
                best_val = chars[i];
            }
        }

        result.push(chars[best_idx]);
        start_idx = best_idx + 1;
    }

    result.parse::<u64>().unwrap_or(0)
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .trim()
            .lines()
            .map(|line| find_max_joltage(line, 2))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .trim()
            .lines()
            .map(|line| find_max_joltage(line, 12))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
