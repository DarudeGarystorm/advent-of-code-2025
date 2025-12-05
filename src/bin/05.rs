use std::ops::RangeInclusive;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.trim().lines();

    // ranges
    let mut ranges: Vec<RangeInclusive<u64>> = [].to_vec();
    for line in lines.by_ref().take_while(|line| !line.is_empty()) {
        let mut nums = line.split('-');
        let start: u64 = nums.next()?.parse().ok()?;
        let end: u64 = nums.next()?.parse().ok()?;
        ranges.push(start..=end);
    }

    // ids
    let mut count: u64 = 0;
    for line in lines {
        let id: u64 = line.parse().ok()?;

        if ranges.iter().any(|r| r.contains(&id)) {
            count += 1;
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.trim().lines();

    // ranges
    let mut ranges: Vec<RangeInclusive<u64>> = [].to_vec();
    for line in lines.by_ref().take_while(|line| !line.is_empty()) {
        let mut nums = line.split('-');
        let start: u64 = nums.next()?.parse().ok()?;
        let end: u64 = nums.next()?.parse().ok()?;
        ranges.push(start..=end);
    }

    ranges.sort_by_key(|r| *r.start());

    let mut merged = vec![ranges[0].clone()];

    for range in &ranges[1..] {
        let last = merged.last_mut().unwrap();

        if *range.start() <= *last.end() + 1 {
            *last = *last.start()..=(*last.end().max(range.end()));
        } else {
            merged.push(range.clone());
        }
    }

    let count = merged.iter().map(|r| r.end() - r.start() + 1).sum();

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
