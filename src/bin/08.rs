advent_of_code::solution!(8);

use itertools::Itertools;
#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
struct Coordinate {
    y: i32,
    x: i32,
}

impl Coordinate {
    fn new(x: i32, y: i32) -> Coordinate {
        Coordinate { y, x }
    }
}

#[allow(dead_code)]
fn print_grid(grid: &[Vec<char>], antinodes: &[Coordinate]) {
    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == '.' && antinodes.contains(&Coordinate::new(x as i32, y as i32)) {
                print!("#");
            } else {
                print!("{}", c);
            }
        }
        println!();
    }
}

#[allow(dead_code)]
fn print_grid_no_antinodes(grid: &[Vec<char>]) {
    for row in grid {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}

fn read_grid(input: &str) -> Vec<Vec<char>> {
    input
        .split_terminator('\n')
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>()
}

fn get_boundary_coordinate(grid: &[Vec<char>]) -> Coordinate {
    assert!(grid.iter().map(|l| l.len()).all_equal());
    Coordinate::new(grid[0].len() as i32 - 1, grid.len() as i32 - 1)
}

fn identify_nodes(grid: &[Vec<char>]) -> Vec<Vec<Coordinate>> {
    let mut nodes: Vec<(char, Vec<Coordinate>)> = Vec::new();
    for (y, line) in grid.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c != '\n' && *c != '.' {
                if let Some(pos) = nodes.iter().position(|(v, _)| v == c) {
                    nodes[pos].1.push(Coordinate::new(x as i32, y as i32));
                } else {
                    nodes.push((*c, vec![Coordinate::new(x as i32, y as i32)]))
                }
            }
        }
    }
    nodes.into_iter().map(|(_, c)| c).collect()
}

fn is_in_boundary(node: Coordinate, boundary: Coordinate) -> bool {
    node.x >= 0 && node.x <= boundary.x && node.y >= 0 && node.y <= boundary.y
}

fn get_antinode_1(far: Coordinate, near: Coordinate, boundary: Coordinate) -> Option<Coordinate> {
    let antinode = Coordinate::new(near.x + (near.x - far.x), near.y + (near.y - far.y));
    if is_in_boundary(antinode, boundary) {
        Some(antinode)
    } else {
        None
    }
}

fn get_antinode_2(far: Coordinate, near: Coordinate, boundary: Coordinate) -> Vec<Coordinate> {
    let mut antinodes = vec![near];
    let dx = near.x - far.x;
    let dy = near.y - far.y;

    let mut x = near.x;
    let mut y = near.y;
    loop {
        let antinode = Coordinate::new(x + dx, y + dy);
        if is_in_boundary(antinode, boundary) {
            antinodes.push(antinode);
            x += dx;
            y += dy;
        } else {
            return antinodes;
        }
    }
}

fn get_antinodes_1(nodes: &[Coordinate], boundary: Coordinate) -> Vec<Coordinate> {
    assert!(nodes.iter().all_unique());
    nodes
        .iter()
        .permutations(2)
        .filter_map(|c| get_antinode_1(*c[0], *c[1], boundary))
        .collect::<Vec<_>>()
}

fn get_antinodes_2(nodes: &[Coordinate], boundary: Coordinate) -> Vec<Coordinate> {
    assert!(nodes.iter().all_unique());
    nodes
        .iter()
        .permutations(2)
        .flat_map(|c| get_antinode_2(*c[0], *c[1], boundary))
        .collect::<Vec<_>>()
}
pub fn part_one(input: &str) -> Option<u32> {
    let grid = read_grid(input);
    let boundary_coordinate = get_boundary_coordinate(&grid);
    let nodes = identify_nodes(&grid);
    Some(
        nodes
            .iter()
            .flat_map(|n| get_antinodes_1(n.as_slice(), boundary_coordinate))
            .unique()
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = read_grid(input);
    let boundary_coordinate = get_boundary_coordinate(&grid);
    let nodes = identify_nodes(&grid);
    Some(
        nodes
            .iter()
            .flat_map(|n| get_antinodes_2(n.as_slice(), boundary_coordinate))
            .unique()
            .count() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
