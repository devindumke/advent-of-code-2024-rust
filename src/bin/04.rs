advent_of_code::solution!(4);

fn parse_line(input: &str) -> Vec<char> {
    input.chars().filter(|&c| !c.is_whitespace()).collect()
}

fn parse_multiline(input: &str) -> Vec<Vec<char>> {
    input
        .split_terminator('\n')
        .collect::<Vec<_>>()
        .iter()
        .map(|&s| parse_line(s))
        .collect::<Vec<_>>()
}
#[derive(Debug, Copy, Clone)]
enum Direction {
    UpperLeft,
    UpperCenter,
    UpperRight,
    MidLeft,
    // MidCenter, Not a direction
    MidRight,
    LowerLeft,
    LowerCenter,
    LowerRight,
}

impl Direction {
    fn all() -> &'static [Direction] {
        &[
            Direction::UpperLeft,
            Direction::UpperCenter,
            Direction::UpperRight,
            Direction::MidLeft,
            Direction::MidRight,
            Direction::LowerLeft,
            Direction::LowerCenter,
            Direction::LowerRight,
        ]
    }
}
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn new(x: usize, y: usize) -> Coordinate {
        Coordinate { x, y }
    }

    fn get_next(&self, direction: Direction) -> Option<Coordinate> {
        match direction {
            Direction::UpperLeft if self.x != 0usize && self.y != usize::MAX => {
                Some(Coordinate::new(self.x - 1usize, self.y + 1usize))
            }
            Direction::UpperCenter if self.y != usize::MAX => {
                Some(Coordinate::new(self.x, self.y + 1usize))
            }
            Direction::UpperRight if self.x != usize::MAX && self.y != usize::MAX => {
                Some(Coordinate::new(self.x + 1usize, self.y + 1usize))
            }
            Direction::MidLeft if self.x != 0usize => {
                Some(Coordinate::new(self.x - 1usize, self.y))
            }
            Direction::MidRight if self.x != usize::MAX => {
                Some(Coordinate::new(self.x + 1usize, self.y))
            }
            Direction::LowerLeft if self.x != 0usize && self.y != 0usize => {
                Some(Coordinate::new(self.x - 1usize, self.y - 1usize))
            }
            Direction::LowerCenter if self.y != 0usize => {
                Some(Coordinate::new(self.x, self.y - 1usize))
            }
            Direction::LowerRight if self.x != usize::MAX && self.y != 0usize => {
                Some(Coordinate::new(self.x + 1usize, self.y - 1usize))
            }
            _ => None,
        }
    }
}

fn get_char_from_grid<'a>(grid: &'a Vec<Vec<char>>, coordinate: &Coordinate) -> Option<&'a char> {
    grid.get(coordinate.y).and_then(|row| row.get(coordinate.x))
}

fn get_xmas_count(grid: &Vec<Vec<char>>) -> u32 {
    let mut count = 0;
    for x in 0..grid.len() {
        'y: for y in 0..grid[0].len() {
            let cord1 = Coordinate::new(x, y);
            if get_char_from_grid(grid, &cord1).unwrap() != &'X' {
                continue 'y;
            }

            'direction: for dir in Direction::all() {
                let cord2 = Coordinate::get_next(&cord1, *dir);
                if cord2.is_none() {
                    continue 'direction;
                }
                let cord2 = cord2.unwrap();
                if Some(&'M') != get_char_from_grid(grid, &cord2) {
                    continue 'direction;
                }

                let cord3 = Coordinate::get_next(&cord2, *dir);
                if cord3.is_none() {
                    continue 'direction;
                }
                let cord3 = cord3.unwrap();
                if Some(&'A') != get_char_from_grid(grid, &cord3) {
                    continue 'direction;
                }

                let cord4 = Coordinate::get_next(&cord3, *dir);
                if cord4.is_none() {
                    continue 'direction;
                }
                let cord4 = cord4.unwrap();
                if Some(&'S') != get_char_from_grid(grid, &cord4) {
                    continue 'direction;
                }
                count += 1u32;
            }
        }
    }
    count
}

fn get_x_mas_count(grid: &Vec<Vec<char>>) -> u32 {
    let mut count = 0;
    for x in 0..grid.len() {
        'y: for y in 0..grid[0].len() {
            let a_cord = Coordinate::new(x, y);
            if get_char_from_grid(grid, &a_cord).unwrap() != &'A' {
                continue 'y;
            }

            let upper_left = Coordinate::get_next(&a_cord, Direction::UpperLeft);
            let upper_right = Coordinate::get_next(&a_cord, Direction::UpperRight);
            let lower_left = Coordinate::get_next(&a_cord, Direction::LowerLeft);
            let lower_right = Coordinate::get_next(&a_cord, Direction::LowerRight);

            if upper_left.is_none()
                || upper_right.is_none()
                || lower_left.is_none()
                || lower_right.is_none()
            {
                continue 'y;
            }

            let upper_left = get_char_from_grid(grid, &upper_left.unwrap());
            let upper_right = get_char_from_grid(grid, &upper_right.unwrap());
            let lower_left = get_char_from_grid(grid, &lower_left.unwrap());
            let lower_right = get_char_from_grid(grid, &lower_right.unwrap());

            if upper_left.is_none()
                || upper_right.is_none()
                || lower_left.is_none()
                || lower_right.is_none()
            {
                continue 'y;
            }

            let upper_left = upper_left.unwrap();
            let upper_right = upper_right.unwrap();
            let lower_left = lower_left.unwrap();
            let lower_right = lower_right.unwrap();

            if upper_left != &'M' && upper_left != &'S' {
                continue 'y;
            }
            if lower_right != &'M' && lower_right != &'S' {
                continue 'y;
            }
            if lower_right == upper_left {
                continue 'y;
            }

            if upper_right != &'M' && upper_right != &'S' {
                continue 'y;
            }
            if lower_left != &'M' && lower_left != &'S' {
                continue 'y;
            }
            if lower_left == upper_right {
                continue 'y;
            }

            count += 1u32;
        }
    }
    count
}
pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_multiline(input);
    Some(get_xmas_count(&grid))
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_multiline(input);
    Some(get_x_mas_count(&grid))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
