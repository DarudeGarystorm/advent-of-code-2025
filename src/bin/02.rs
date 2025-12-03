advent_of_code::solution!(2);

fn parse_range(range: &str) -> Option<(u64, u64)> {
    let (s, e) = range.split_once('-')?;
    let start: u64 = s.parse().ok()?;
    let end: u64 = e.parse().ok()?;
    Some((start, end))
}

pub fn part_one(input: &str) -> Option<u64> {
    input.trim().split(',').try_fold(0u64, |acc, range| {
        let (start, end) = parse_range(range)?;

        let inner_sum = (start..=end).try_fold(0u64, |acc, num| {
            let s = num.to_string();
            let mid = s.len() / 2;
            let (left, right) = s.split_at(mid);

            if left == right {
                Some(acc + num)
            } else {
                Some(acc)
            }
        })?;

        Some(acc + inner_sum)
    })
}

pub fn part_two(input: &str) -> Option<u64> {
    input.trim().split(',').try_fold(0u64, |acc, range| {
        let (start, end) = parse_range(range)?;

        let inner_sum = (start..=end).try_fold(0u64, |acc, num| {
            let s = num.to_string();

            if is_repeated_at_least_twice(&s) {
                Some(acc + num)
            } else {
                Some(acc)
            }
        })?;

        Some(acc + inner_sum)
    })
}

fn is_repeated_at_least_twice(s: &str) -> bool {
    let n = s.len();
    // single-digit numbers can't repeat at least twice
    if n < 2 {
        return false;
    }

    let bytes = s.as_bytes();

    for k in 1..=n / 2 {
        if !n.is_multiple_of(k) {
            continue;
        }
        let pat = &bytes[..k];

        if bytes.chunks_exact(k).all(|chunk| chunk == pat) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
