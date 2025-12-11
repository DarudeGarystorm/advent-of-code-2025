use std::collections::HashMap;

advent_of_code::solution!(11);

fn count_problematic_paths(
    current: &str,
    graph: &HashMap<String, Vec<String>>,
    memo: &mut HashMap<String, (u64, u64, u64, u64)>,
) -> (u64, u64, u64, u64) {
    if current == "out" {
        return (0, 1, 0, 0);
    }

    if let Some(&cached) = memo.get(current) {
        return cached;
    }

    let is_dac = current == "dac";
    let is_fft = current == "fft";
    let mut both_acc = 0u64;
    let mut total_acc = 0u64;
    let mut dac_acc = 0u64;
    let mut fft_acc = 0u64;

    if let Some(neighbors) = graph.get(current) {
        for neighbor in neighbors {
            let (b, t, d, f) = count_problematic_paths(neighbor, graph, memo);

            // always add the total
            total_acc += t;

            // paths that contain dac/fft so far
            // promote if current is dac/fft
            let dac_promoted = d + if is_dac { t } else { 0 };
            let fft_promoted = f + if is_fft { t } else { 0 };

            dac_acc += dac_promoted;
            fft_acc += fft_promoted;

            // paths that contain both
            // 1. already both in subtree
            // 2. if current dac, any subtree with fft
            // 3. if current fft, any subtree with dac
            let mut both_promoted = b;
            if is_dac {
                both_promoted += f;
            }
            if is_fft {
                both_promoted += d;
            }
            both_acc += both_promoted;
        }
    }

    let result = (both_acc, total_acc, dac_acc, fft_acc);
    memo.insert(current.to_string(), result);
    result
}

fn count_paths(
    current: &str,
    graph: &HashMap<String, Vec<String>>,
    memo: &mut HashMap<String, u64>,
) -> u64 {
    if current == "out" {
        return 1;
    }

    if let Some(&cached) = memo.get(current) {
        return cached;
    }

    let total = graph
        .get(current)
        .map(|neighbors| {
            neighbors
                .iter()
                .map(|neighbor| count_paths(neighbor, graph, memo))
                .sum()
        })
        .unwrap_or(0);

    memo.insert(current.to_string(), total);
    total
}

fn parse_input(input: &str) -> HashMap<String, Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    input.trim().lines().for_each(|line| {
        let parts: Vec<&str> = line.split_whitespace().collect();

        let start = parts[0].trim_end_matches(":").to_string();

        let neighbors = parts[1..].iter().map(|s| s.to_string()).collect();
        map.insert(start, neighbors);
    });
    map
}

pub fn part_one(input: &str) -> Option<u64> {
    let map = parse_input(input);
    let mut memo: HashMap<String, u64> = HashMap::new();
    Some(count_paths("you", &map, &mut memo))
}

pub fn part_two(input: &str) -> Option<u64> {
    let map = parse_input(input);
    let mut memo: HashMap<String, (u64, u64, u64, u64)> = HashMap::new();
    Some(count_problematic_paths("svr", &map, &mut memo).0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2));
    }
}
