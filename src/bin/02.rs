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
fn is_safe(report: &[u32], use_dampener: bool) -> bool {
    match use_dampener {
        false => {
            find_unsafe_level(report, Direction::Increasing).is_none()
                || find_unsafe_level(report, Direction::Decreasing).is_none()
        }
        true => {
            is_safe_with_dampener(report, Direction::Increasing)
                || is_safe_with_dampener(report, Direction::Decreasing)
        }
    }
}

fn is_safe_with_dampener(report: &[u32], direction: Direction) -> bool {
    let bad_level_index = find_unsafe_level(report, direction);
    if bad_level_index.is_none() {
        return true;
    }
    let bad_level_index = bad_level_index.unwrap();
    for possible_bad_level in
        bad_level_index.saturating_sub(1usize)..bad_level_index.saturating_add(1usize)
    {
        if try_remove_level(report, possible_bad_level, direction) {
            return true;
        }
    }
    false
}

fn try_remove_level(report: &[u32], level_to_remove: usize, direction: Direction) -> bool {
    if level_to_remove >= report.len() {
        return false;
    }
    let mut report = report.to_vec();
    report.remove(level_to_remove);
    find_unsafe_level(&report, direction).is_none()
}

fn find_unsafe_level(report: &[u32], direction: Direction) -> Option<usize> {
    let mut prev_level: Option<u32> = None;
    for (index, level) in report.iter().enumerate() {
        if prev_level.is_none() {
            prev_level = Some(*level);
            continue;
        }
        let difference = get_difference(prev_level.unwrap(), *level, direction);
        if difference.is_none_or(|x| !(1..=3).contains(&x)) {
            return Some(index);
        }
        prev_level = Some(*level);
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
