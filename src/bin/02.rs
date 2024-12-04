advent_of_code::solution!(2);

fn parse_report_into_vec(input: &str) -> Vec<u32> {
    input
        .split_whitespace()
        .map(|num| num.parse::<u32>().unwrap())
        .collect()
}

fn read_reports(input: &str) -> Vec<Vec<u32>> {
    input
        .split_terminator('\n')
        .collect::<Vec<_>>()
        .iter()
        .map(|&s| parse_report_into_vec(s))
        .collect::<Vec<_>>()
}

#[derive(Clone, Copy)]
enum Direction {
    Increasing,
    Decreasing,
}

fn get_difference(prev: u32, curr: u32, direction: Direction) -> Option<u32> {
    match direction {
        Direction::Increasing => curr.checked_sub(prev),
        Direction::Decreasing => prev.checked_sub(curr),
    }
}
fn is_safe(vec: &Vec<u32>, use_dampener: bool) -> bool {
    match use_dampener {
        false => {
            find_unsafe_level(vec, Direction::Increasing).is_none()
                || find_unsafe_level(vec, Direction::Decreasing).is_none()
        }
        true => {
            is_safe_with_dampener(vec, Direction::Increasing)
                || is_safe_with_dampener(vec, Direction::Decreasing)
        }
    }
}

fn is_safe_with_dampener(vec: &Vec<u32>, direction: Direction) -> bool {
    let bad_level_index = find_unsafe_level(vec, direction);
    if bad_level_index.is_none() {
        return true;
    }
    let bad_level_index = bad_level_index.unwrap();
    for element_to_remove in
        bad_level_index.saturating_sub(1usize)..bad_level_index.saturating_add(1usize)
    {
        if try_remove_element(vec, element_to_remove, direction) {
            return true;
        }
    }
    false
}

fn try_remove_element(vec: &Vec<u32>, element_to_remove: usize, direction: Direction) -> bool {
    if element_to_remove >= vec.len() {
        return false;
    }
    let mut vec = vec.clone();
    vec.remove(element_to_remove);
    find_unsafe_level(&vec, direction).is_none()
}

fn find_unsafe_level(vec: &Vec<u32>, direction: Direction) -> Option<usize> {
    let mut prev_value: Option<u32> = None;
    for (index, val) in vec.iter().enumerate() {
        if prev_value.is_none() {
            prev_value = Some(*val);
            continue;
        }
        let difference = get_difference(prev_value.unwrap(), *val, direction);
        if difference.is_none_or(|x| !(1..=3).contains(&x)) {
            return Some(index);
        }
        prev_value = Some(*val);
    }
    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let reports = read_reports(input);
    Some(reports.iter().filter(|&v| is_safe(v, false)).count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let reports = read_reports(input);
    Some(reports.iter().filter(|&v| is_safe(v, true)).count() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
