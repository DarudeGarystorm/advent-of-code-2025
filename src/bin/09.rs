advent_of_code::solution!(9);

fn get_red_tiles(input: &str) -> Vec<(u64, u64)> {
    input
        .trim()
        .lines()
        .filter_map(|line| {
            let mut nums = line.split(',');
            let x: u64 = nums.next()?.parse().ok()?;
            let y: u64 = nums.next()?.parse().ok()?;

            Some((x, y))
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let red_tiles = get_red_tiles(input);
    let mut max_area = 0;

    for i in 0..red_tiles.len() {
        for j in (i + 1)..red_tiles.len() {
            let (x1, y1) = red_tiles[i];
            let (x2, y2) = red_tiles[j];

            let width = x2.abs_diff(x1) + 1;
            let height = y2.abs_diff(y1) + 1;
            let area = width * height;

            max_area = max_area.max(area);
        }
    }

    Some(max_area)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
