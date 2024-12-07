advent_of_code::solution!(6);

use itertools::Itertools;

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
enum State {
    GuardStartingPosition,
    Obstacle,
    Open,
}

impl State {
    fn from(c: &char) -> Option<State> {
        match c {
            '^' => Some(State::GuardStartingPosition),
            '#' => Some(State::Obstacle),
            '.' => Some(State::Open),
            _ => None,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn turn_clockwise(&self) -> Direction {
        match self {
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn new(x: usize, y: usize) -> Coordinate {
        Coordinate { x, y }
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Map {
    map: Vec<Vec<State>>,
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
struct Status {
    coordinate: Coordinate,
    direction: Direction,
}

impl Status {
    fn new(coordinate: Coordinate, direction: Direction) -> Status {
        Status {
            coordinate,
            direction,
        }
    }

    fn next_status(&self) -> Option<Status> {
        match self.direction {
            Direction::Left if self.coordinate.x != 0usize => Some(Status::new(
                Coordinate::new(self.coordinate.x - 1usize, self.coordinate.y),
                self.direction,
            )),
            Direction::Right if self.coordinate.x != usize::MAX => Some(Status::new(
                Coordinate::new(self.coordinate.x + 1usize, self.coordinate.y),
                self.direction,
            )),
            Direction::Up if self.coordinate.y != 0usize => Some(Status::new(
                Coordinate::new(self.coordinate.x, self.coordinate.y - 1usize),
                self.direction,
            )),
            Direction::Down if self.coordinate.y != usize::MAX => Some(Status::new(
                Coordinate::new(self.coordinate.x, self.coordinate.y + 1usize),
                self.direction,
            )),
            _ => None,
        }
    }

    fn turn_clockwise(&self) -> Status {
        Status::new(self.coordinate, self.direction.turn_clockwise())
    }
}

impl Map {
    fn new(map: Vec<Vec<State>>) -> Map {
        Map { map }
    }

    fn get_guard_starting_position(&self) -> Option<Coordinate> {
        for (y, row) in self.map.iter().enumerate() {
            for (x, state) in row.iter().enumerate() {
                if *state == State::GuardStartingPosition {
                    return Some(Coordinate::new(x, y));
                }
            }
        }
        None
    }

    fn get_state(&self, coord: &Coordinate) -> Option<&State> {
        self.map.get(coord.y).and_then(|row| row.get(coord.x))
    }

    fn place_obstacle(&mut self, coord: &Coordinate) {
        self.map[coord.y][coord.x] = State::Obstacle;
    }
}
fn read_line(line: &str) -> Vec<State> {
    line.chars()
        .map(|c| State::from(&c).expect("All characters in line can be parsed to a state"))
        .collect::<Vec<_>>()
}
fn read_map(input: &str) -> Vec<Vec<State>> {
    input.lines().map(read_line).collect()
}

fn get_unique_coordinates(prev_moves: &[Status]) -> Vec<Coordinate> {
    prev_moves
        .iter()
        .map(|&s| s.coordinate)
        .unique()
        .collect::<Vec<_>>()
}

fn is_infinite_loop(prev_moves: &[Status], status: Status) -> bool {
    prev_moves.contains(&status)
}

fn register_status_in_log(prev_moves: &mut Vec<Status>, status: Status) {
    prev_moves.push(status);
}

fn move_guard_forward(map: &Map, guard_status: Status) -> Option<Status> {
    let next_status = guard_status.next_status()?;
    let next_coordinate_state = map.get_state(&next_status.coordinate)?;
    if next_coordinate_state == &State::Obstacle {
        return move_guard_forward(map, guard_status.turn_clockwise());
    }
    Some(next_status)
}
fn trace_guard_path(map: &Map, init_status: Status) -> Option<Vec<Coordinate>> {
    let mut guard_status = init_status;
    let mut prev_moves = Vec::new();

    loop {
        if is_infinite_loop(prev_moves.as_slice(), guard_status) {
            return None;
        }
        register_status_in_log(&mut prev_moves, guard_status);
        if let Some(next) = move_guard_forward(map, guard_status) {
            guard_status = next;
        } else {
            return Some(get_unique_coordinates(prev_moves.as_slice()));
        }
    }
}

fn place_obstacle(map: &Map, coordinate: &Coordinate) -> Map {
    let mut new_map = map.clone();
    new_map.place_obstacle(coordinate);
    new_map
}

fn get_infinite_loops(map: &Map, init_status: Status, prev_path: &[Coordinate]) -> u32 {
    let mut count = 0;
    for coordinate in prev_path.iter().filter(|p| init_status.coordinate != **p) {
        // NOTE: big slowdown here due to copying the map each time.
        //       a faster solution would be to do everything on the first pass
        //       simulating an obstacle by rotating the guard clockwise
        //       and testing for infinite loops every time the guard is about to take a step
        let map = place_obstacle(map, coordinate);
        if trace_guard_path(&map, init_status).is_none() {
            count += 1;
        }
    }
    count
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = Map::new(read_map(input));
    let init_guard_coordinate = map
        .get_guard_starting_position()
        .expect("map to start with a guard on it");
    let init_guard_status = Status::new(init_guard_coordinate, Direction::Up);
    let path = trace_guard_path(&map, init_guard_status).expect("finite path");
    Some(path.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = Map::new(read_map(input));
    let init_guard_coordinate = map
        .get_guard_starting_position()
        .expect("map to start with a guard on it");
    let init_guard_status = Status::new(init_guard_coordinate, Direction::Up);
    let path = trace_guard_path(&map, init_guard_status).expect("finite path");
    let infinite_loop_count = get_infinite_loops(&map, init_guard_status, &path);
    Some(infinite_loop_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
