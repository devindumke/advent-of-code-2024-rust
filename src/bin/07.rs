advent_of_code::solution!(7);

enum Operation {
    Multiply,
    Add,
    Concatenate,
}

impl Operation {
    fn no_concatenate() -> &'static [Operation] {
        &[Operation::Multiply, Operation::Add]
    }

    fn all() -> &'static [Operation] {
        &[Operation::Concatenate, Operation::Multiply, Operation::Add]
    }

    fn reverse_operation(&self, result: u64, value: u64) -> Option<u64> {
        match self {
            Operation::Multiply => (result % value == 0).then(|| result / value),
            Operation::Add => result.checked_sub(value),
            Operation::Concatenate => {
                let value_digit_count = (value as f64).log10().floor() as u64 + 1;
                let result_digit_count = (result as f64).log10().floor() as u64 + 1;
                if value_digit_count >= result_digit_count {
                    return None;
                }
                let divisor = 10u64.pow(value_digit_count as u32);
                if result % divisor != value {
                    return None;
                }
                Some(result / 10u64.pow(value_digit_count as u32))
            }
        }
    }
}
fn read_equation(line: &str) -> (u64, Vec<u64>) {
    let mut parts = line.split(|c: char| !c.is_numeric());
    let result = parts.next().unwrap().parse::<u64>().unwrap();
    let values = parts.filter_map(|s| s.parse().ok()).collect::<Vec<u64>>();
    (result, values)
}

fn read_equations(input: &str) -> Vec<(u64, Vec<u64>)> {
    input.lines().map(read_equation).collect()
}

fn equation_is_solvable_1(result: u64, values: &[u64]) -> bool {
    assert!(!values.is_empty());
    if values.len() == 1 {
        return result == values[0];
    }
    let last_operand = values.last().unwrap();
    let remaining_operands = &values[..values.len() - 1];
    Operation::no_concatenate()
        .iter()
        .filter_map(|o| o.reverse_operation(result, *last_operand))
        .any(|r| equation_is_solvable_1(r, remaining_operands))
}

fn equation_is_solvable_2(result: u64, values: &[u64]) -> bool {
    assert!(!values.is_empty());
    if values.len() == 1 {
        return result == values[0];
    }
    let last_operand = values.last().unwrap();
    let remaining_operands = &values[..values.len() - 1];

    Operation::all()
        .iter()
        .filter_map(|o| o.reverse_operation(result, *last_operand))
        .any(|r| equation_is_solvable_2(r, remaining_operands))
}

pub fn part_one(input: &str) -> Option<u64> {
    let equations = read_equations(input);
    let solvable_equations = equations
        .iter()
        .filter(|(r, v)| equation_is_solvable_1(*r, v.as_slice()));
    let sum = solvable_equations.map(|s| s.0).sum::<u64>();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let equations = read_equations(input);
    let solvable_equations = equations
        .iter()
        .filter(|(r, v)| equation_is_solvable_2(*r, v.as_slice()));
    let sum = solvable_equations.map(|s| s.0).sum::<u64>();
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
