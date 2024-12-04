advent_of_code::solution!(1);

fn parse_line_into_tuple(line: &str) -> (u32, u32) {
    let mut parts = line.split_whitespace();
    let first = parts.next().unwrap().parse::<u32>().unwrap();
    let second = parts.next().unwrap().parse::<u32>().unwrap();
    (first, second)
}
fn read_list(input: &str) -> Vec<(u32, u32)> {
    let lines = input.split_terminator('\n').collect::<Vec<_>>();
    let values = lines
        .iter()
        .map(|&s| parse_line_into_tuple(s))
        .collect::<Vec<_>>();
    values
}

fn sort_vec(vec: Vec<(u32, u32)>) -> Vec<(u32, u32)> {
    let (mut first_vec, mut second_vec): (Vec<u32>, Vec<u32>) = vec.into_iter().unzip();
    first_vec.sort();
    second_vec.sort();
    first_vec
        .iter()
        .zip(second_vec.iter())
        .map(|(a, b)| (*a, *b))
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let list = read_list(input);
    let inputs = sort_vec(list);
    let distances = inputs
        .iter()
        .map(|&(a, b)| a.abs_diff(b))
        .collect::<Vec<_>>();
    Some(distances.iter().sum::<u32>())
}

pub fn part_two(input: &str) -> Option<u32> {
    let list = read_list(input);
    let (first_vec, second_vec): (Vec<u32>, Vec<u32>) = list.into_iter().unzip();
    let mut similarity_score: u32 = 0;
    for element in first_vec {
        similarity_score += element * (second_vec.iter().filter(|&n| *n == element).count() as u32);
    }
    Some(similarity_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
