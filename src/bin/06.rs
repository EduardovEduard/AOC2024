use std::collections::HashSet;
use itertools::Itertools;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

advent_of_code::solution!(6);

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone, EnumIter)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Direction::N => Direction::E,
            Direction::E => Direction::S,
            Direction::S => Direction::W,
            Direction::W => Direction::N,
        }
    }

    fn opposite(&self) -> Self {
        match self {
            Direction::N => Direction::S,
            Direction::E => Direction::W,
            Direction::S => Direction::N,
            Direction::W => Direction::E,
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
struct MovingPoint {
    x: i32,
    y: i32,
    direction: Direction,
}

impl MovingPoint {
    fn move_once(&self) -> MovingPoint {
        let new_point = Point { x: self.x, y: self.y }
            .move_to(&self.direction);
        MovingPoint {
            x: new_point.x,
            y: new_point.y,
            direction: self.direction,
        }
    }

    fn move_back(&self) -> MovingPoint {
        let new_point = Point { x: self.x, y: self.y }
            .move_to(&self.direction.opposite());
        MovingPoint {
            x: new_point.x,
            y: new_point.y,
            direction: self.direction,
        }
    }

    fn turn_right(&self) -> MovingPoint {
        MovingPoint {
            x: self.x,
            y: self.y,
            direction: self.direction.turn_right(),
        }
    }
}

impl From<MovingPoint> for Point {
    fn from(value: MovingPoint) -> Self {
        Self { x: value.x, y: value.y }
    }
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn move_to(&self, dir: &Direction) -> Self {
        match dir {
            Direction::N => Point { x: self.x, y: self.y - 1 },
            Direction::E => Point { x: self.x + 1, y: self.y },
            Direction::S => Point { x: self.x, y: self.y + 1 },
            Direction::W => Point { x: self.x - 1, y: self.y },
        }
    }
}

struct Board {
    board: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Char {
    DOT,
    HASH,
    GUARD(Direction),
}

impl Char {
    fn from_char(ch: char) -> Self {
        match ch {
            '#' => Char::HASH,
            '.' => Char::DOT,
            '^' => Char::GUARD(Direction::N),
            '>' => Char::GUARD(Direction::E),
            'v' => Char::GUARD(Direction::S),
            '<' => Char::GUARD(Direction::W),
            _ => panic!("Invalid char: {}", ch),
        }
    }
}

impl Board {
    fn new(input: &str) -> Self {
        let board: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let width = board[0].len();
        let height = board.len();
        Self {
            board,
            width,
            height,
        }
    }

    fn get(&self, point: &Point) -> Option<Char> {
        if point.x < 0 || point.y < 0 || point.x >= self.width as i32 || point.y >= self.height as i32 {
            return None;
        }
        Some(Char::from_char(self.board[point.y as usize][point.x as usize]))
    }

    fn set(&mut self, point: &Point, char: char) {
        self.board[point.y as usize][point.x as usize] = char;
    }

    fn guard_position(&self) -> MovingPoint {
        for y in 0..self.height {
            for x in 0..self.width {
                let point = Point::new(x as i32, y as i32);
                if let g @ Char::GUARD(direction) = self.get(&point).unwrap() {
                    return MovingPoint {
                        x: point.x,
                        y: point.y,
                        direction,
                    };
                }
            }
        }
        panic!("No guard found");
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let board = Board::new(input);
    let mut point = board.guard_position();
    let mut positions: HashSet<Point> = HashSet::new();
    positions.insert(point.into());

    while !looking_away(&board, &point) {
        let new_point = point.move_once();
        if board.get(&new_point.into()) == Some(Char::HASH) {
            point.direction = point.direction.turn_right();
        } else {
            point = new_point;
        }
        positions.insert(point.into());
    }

    Some(positions.len() as u32)
}

fn looking_away(board: &Board, point: &MovingPoint) -> bool {
    point.x == 0 && point.direction == Direction::W ||
        point.x == board.width as i32 - 1 && point.direction == Direction::E ||
        point.y == 0 && point.direction == Direction::N ||
        point.y == board.height as i32 - 1 && point.direction == Direction::S
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut board = Board::new(input);
    let guard_start = board.guard_position();
    let mut positions: HashSet<Point> = HashSet::new();
    let mut point = guard_start.clone();
    positions.insert(point.into());

    while !looking_away(&board, &point) {
        let new_point = point.move_once();
        if board.get(&new_point.into()) == Some(Char::HASH) {
            point = point.turn_right();
        } else {
            point = new_point;
        }
        positions.insert(point.into());
    }

    positions.remove(&guard_start.into());

    Some(positions.iter().filter(|point|
        can_place_obstacle(&board, guard_start, &point)
    ).count() as u32)
}

fn can_place_obstacle(board: &Board, mut guard: MovingPoint, position: &Point) -> bool {
    let mut loop_visited = HashSet::new();
    while !loop_visited.contains(&guard) {
        if looking_away(&board, &guard) {
            return false;
        }
        loop_visited.insert(guard);
        let step = guard.move_once();
        if board.get(&step.into()) == Some(Char::HASH) || (step.x == position.x && step.y == position.y) {
            guard = guard.turn_right();
        } else {
            guard = step;
        }
    }
    true
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
