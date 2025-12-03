advent_of_code::solution!(1);

const DIAL_SIZE: i64 = 100;
const STARTING_POSITION: i64 = 50;

/// Parse a rotation instruction ("L50" or "R30")
fn parse_rotation(line: &str) -> Option<(char, i64)> {
    let direction = line.chars().next()?;
    let distance = line.get(1..)?.parse::<i64>().ok()?;
    Some((direction, distance))
}

/// Calculate new position after a rotation (unnormalized)
fn apply_rotation(position: i64, direction: char, distance: i64) -> i64 {
    match direction {
        'L' => position - distance,
        'R' => position + distance,
        _ => position,
    }
}

/// Calculate normalized dial position (0-99) after a rotation
fn apply_rotation_normalized(position: i64, direction: char, distance: i64) -> i64 {
    apply_rotation(position, direction, distance).rem_euclid(DIAL_SIZE)
}

/// Count how many times we point at 0 during a rotation (inclusive of endpoint).
fn count_zero_crossings(start_pos: i64, direction: char, distance: i64) -> u64 {
    let start = start_pos % DIAL_SIZE; // normalize

    let clicks_to_next_zero = match direction {
        'R' => {
            if start == 0 {
                DIAL_SIZE
            } else {
                DIAL_SIZE - start
            }
        }
        'L' => {
            if start == 0 {
                DIAL_SIZE
            } else {
                start
            }
        }
        _ => return 0,
    };

    if distance < clicks_to_next_zero {
        0
    } else {
        1 + ((distance - clicks_to_next_zero) / DIAL_SIZE) as u64
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    input
        .lines()
        .try_fold((STARTING_POSITION, 0u64), |(pos, count), line| {
            let (direction, distance) = parse_rotation(line)?;
            let new_pos = apply_rotation_normalized(pos, direction, distance);
            Some((new_pos, count + (new_pos == 0) as u64))
        })
        .map(|(_, count)| count)
}

pub fn part_two(input: &str) -> Option<u64> {
    input
        .lines()
        .try_fold((STARTING_POSITION, 0u64), |(pos, count), line| {
            let (direction, distance) = parse_rotation(line)?;

            let crossings = count_zero_crossings(pos, direction, distance);
            let new_pos = apply_rotation_normalized(pos, direction, distance);

            Some((new_pos, count + crossings))
        })
        .map(|(_, count)| count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crossings_right_no_wrap() {
        assert_eq!(count_zero_crossings(50, 'R', 30), 0);
    }

    #[test]
    fn crossings_right_land_on_zero() {
        assert_eq!(count_zero_crossings(50, 'R', 250), 3);
    }

    #[test]
    fn crossings_right_multi_wrap() {
        assert_eq!(count_zero_crossings(50, 'R', 1000), 10);
    }

    #[test]
    fn crossings_left_no_reach_zero() {
        assert_eq!(count_zero_crossings(52, 'L', 30), 0);
    }

    #[test]
    fn crossings_left_hit_once() {
        assert_eq!(count_zero_crossings(50, 'L', 68), 1);
    }

    #[test]
    fn crossings_left_land_on_zero() {
        assert_eq!(count_zero_crossings(50, 'L', 250), 3);
    }

    #[test]
    fn crossings_left_from_zero() {
        assert_eq!(count_zero_crossings(0, 'L', 250), 2);
    }

    #[test]
    fn crossings_right_from_zero() {
        assert_eq!(count_zero_crossings(0, 'R', 250), 2);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn part2_left_250() {
        assert_eq!(part_two("L250"), Some(3));
    }

    #[test]
    fn part2_right_250() {
        assert_eq!(part_two("R250"), Some(3));
    }

    #[test]
    fn part2_left_250_then_right_300() {
        assert_eq!(part_two("L250\nR300"), Some(6));
    }

    #[test]
    fn part2_right_300_then_left_250() {
        assert_eq!(part_two("R300\nL250"), Some(6));
    }

    #[test]
    fn part2_start_at_zero_then_right_200() {
        assert_eq!(part_two("L50\nR200"), Some(3));
    }

    #[test]
    fn part2_start_at_zero_then_left_200() {
        assert_eq!(part_two("R50\nL200"), Some(3));
    }
}
