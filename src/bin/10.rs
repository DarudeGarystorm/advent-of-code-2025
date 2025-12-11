use std::collections::HashMap;

advent_of_code::solution!(10);

struct Machine {
    target_light: String,
    buttons: Vec<Vec<usize>>,
    _joltage: Vec<u64>,
}

fn parse_input(input: &str) -> Vec<Machine> {
    input.trim().lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Machine {
    let parts: Vec<&str> = line.split_whitespace().collect();

    // Extract target light pattern from [.##.]
    let target_light = parts[0].trim_matches(|c| c == '[' || c == ']').to_string();

    // Extract buttons - everything between the first and last parts
    let mut buttons = Vec::new();
    let mut i = 1;
    while i < parts.len() && !parts[i].starts_with('{') {
        let button_str = parts[i].trim_matches(|c| c == '(' || c == ')');
        let button: Vec<usize> = button_str.split(',').map(|s| s.parse().unwrap()).collect();
        buttons.push(button);
        i += 1;
    }

    // Extract joltage
    let joltage_str = parts[i].trim_matches(|c| c == '{' || c == '}');
    let joltage: Vec<u64> = joltage_str.split(',').map(|s| s.parse().unwrap()).collect();

    Machine {
        target_light,
        buttons,
        _joltage: joltage,
    }
}

fn solve_lights(machine: &Machine) -> Option<u64> {
    let initial_light = ".".repeat(machine.target_light.len());
    let mut memo: HashMap<String, u64> = HashMap::new();
    memo.insert(initial_light.clone(), 0);

    fn f(
        current_light: &str,
        target_light: &str,
        buttons: &[Vec<usize>],
        memo: &mut HashMap<String, u64>,
    ) {
        let current_steps = *memo.get(current_light).unwrap();

        for button in buttons {
            let mut next_light = current_light.chars().collect::<Vec<char>>();
            for &toggle in button {
                let c = next_light[toggle];
                if c == '.' {
                    next_light[toggle] = '#';
                } else if c == '#' {
                    next_light[toggle] = '.';
                } else {
                    unreachable!()
                }
            }
            let next_light_str: String = next_light.into_iter().collect();

            if !memo.contains_key(&next_light_str) || memo[&next_light_str] > current_steps + 1 {
                memo.insert(next_light_str.clone(), current_steps + 1);

                if next_light_str != target_light {
                    f(&next_light_str, target_light, buttons, memo);
                }
            }
        }
    }

    println!("STARTING {:?}", machine.target_light);

    f(
        &initial_light,
        &machine.target_light,
        &machine.buttons,
        &mut memo,
    );

    println!(
        "SOLVED {:?} in {:?}",
        machine.target_light,
        memo.get(&machine.target_light)
    );

    memo.get(&machine.target_light).copied()
}

fn solve_joltage(_machine: &Machine) -> Option<u64> {
    None
}

pub fn part_one(input: &str) -> Option<u64> {
    let machines = parse_input(input);
    let total: u64 = machines.iter().filter_map(solve_lights).sum();
    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let machines = parse_input(input);
    let total: u64 = machines.iter().filter_map(solve_joltage).sum();
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }
}
