advent_of_code::solution!(3);

use regex::Regex;

#[derive(Debug, Clone, Copy)]
enum Operation {
    Multiply(u32, u32),
    Do,
    Dont,
}

fn get_operations(haystack: &str, re: &str) -> Vec<Operation> {
    let re = Regex::new(re).unwrap();
    let mut operations = Vec::new();
    for needle in re.captures_iter(haystack) {
        if let Some(left) = needle.get(1) {
            let left = left.as_str().parse::<u32>().unwrap();
            let right = needle.get(2).unwrap().as_str().parse::<u32>().unwrap();
            operations.push(Operation::Multiply(left, right));
        } else if needle.get(3).is_some() {
            operations.push(Operation::Do);
        } else if needle.get(4).is_some() {
            operations.push(Operation::Dont);
        } else {
            panic!("unexpected capture");
        }
    }
    operations
}

fn filter_operations(operations: &Vec<Operation>) -> Vec<(u32, u32)> {
    let mut ret_val = Vec::new();
    let mut enable_operations = true;
    for operation in operations {
        match operation {
            Operation::Multiply(left, right) => {
                if enable_operations {
                    ret_val.push((*left, *right));
                }
            }
            Operation::Do => enable_operations = true,
            Operation::Dont => enable_operations = false,
        }
    }
    ret_val
}

fn calculate_result(haystack: &str, re: &str) -> u32 {
    let operations = get_operations(haystack, re);
    let operations = filter_operations(&operations);
    operations.iter().map(|o| o.0 * o.1).sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    let re = r"mul\((\d{1,3}),(\d{1,3})\)";
    Some(calculate_result(input, re))
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = r"mul\((\d{1,4}),(\d{1,4})\)|(do\(\))|(don't\(\))";
    Some(calculate_result(input, re))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
