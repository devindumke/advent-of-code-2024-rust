advent_of_code::solution!(5);

use regex::Regex;

fn parse_rules(input: &str) -> Vec<(u32, u32)> {
    let re = r"(\d+)\|(\d+)";
    let re = Regex::new(re).unwrap();
    let mut rules = Vec::new();

    for needles in re.captures_iter(input) {
        let Ok(left) = needles[1].parse::<u32>() else {
            continue;
        };
        let Ok(right) = needles[2].parse::<u32>() else {
            continue;
        };
        rules.push((left, right));
    }
    rules
}

fn parse_update(input: &str) -> Vec<u32> {
    let re = r"(\d+)";
    let re = Regex::new(re).unwrap();
    re.captures_iter(input)
        .map(|c| c[1].parse::<u32>().unwrap())
        .collect()
}
fn parse_updates(input: &str) -> Vec<Vec<u32>> {
    let re = r"(.{6,})";
    let re = Regex::new(re).unwrap();
    re.captures_iter(input)
        .map(|c| parse_update(&c[1]))
        .collect()
}

fn get_valid_updates(rules: &[(u32, u32)], updates: &[Vec<u32>]) -> Vec<Vec<u32>> {
    updates
        .iter()
        .filter(|u| get_invalid_index(rules, u).is_none())
        .cloned()
        .collect()
}

fn get_invalid_updates(rules: &[(u32, u32)], updates: &[Vec<u32>]) -> Vec<Vec<u32>> {
    updates
        .iter()
        .filter(|u| get_invalid_index(rules, u).is_some())
        .cloned()
        .collect()
}

fn get_invalid_index(rules: &[(u32, u32)], update: &[u32]) -> Option<(usize, u32)> {
    for (index, value) in update.iter().enumerate() {
        let update = &update[index..];
        if rules
            .iter()
            .filter(|&(_, v)| v == value)
            .filter(|&r| update.contains(&r.0))
            .count()
            > 0
        {
            return Some((index, *value));
        }
    }
    None
}

fn reorder_update(rules: &[(u32, u32)], update: &mut [u32]) {
    let Some((invalid_index, invalid_value)) = get_invalid_index(rules, update) else {
        return; // in order
    };

    let swap_index = rules
        .iter()
        .filter(|&(_, rhs)| *rhs == invalid_value)
        .find(|&(lhs, _)| update[invalid_index..].contains(lhs))
        .map(|(lhs, _)| lhs)
        .and_then(|value| update.iter().position(|r| *r == *value))
        .expect("should be at least 1 value in the list whose order violates the rule");

    update.swap(invalid_index, swap_index);
    reorder_update(rules, update);
}
fn reorder_updates(rules: &[(u32, u32)], updates: &mut [Vec<u32>]) {
    for update in updates.iter_mut() {
        reorder_update(rules, update);
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let rules = parse_rules(input);
    let updates = parse_updates(input);
    let updates = get_valid_updates(&rules, &updates);
    Some(updates.iter().map(|u| u[u.len() / 2]).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let rules = parse_rules(input);
    let updates = parse_updates(input);
    let mut updates = get_invalid_updates(&rules, &updates);
    reorder_updates(&rules, &mut updates);
    Some(updates.iter().map(|u| u[u.len() / 2]).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
