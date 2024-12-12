use std::collections::{HashSet, VecDeque};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

advent_of_code::solution!(10);

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone, EnumIter)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl Point {
    fn move_to(&self, dir: &Direction) -> Self {
        match dir {
            Direction::N => Point {
                x: self.x,
                y: self.y - 1,
            },
            Direction::E => Point {
                x: self.x + 1,
                y: self.y,
            },
            Direction::S => Point {
                x: self.x,
                y: self.y + 1,
            },
            Direction::W => Point {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
}

struct Board {
    board: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

impl Board {
    fn new(input: &str) -> Self {
        let board: Vec<Vec<u8>> = input
            .lines()
            .map(|line| line.chars().map(|c| (c as u8) - b'0').collect())
            .collect();
        let width = board[0].len();
        let height = board.len();
        Self {
            board,
            width,
            height,
        }
    }

    fn get(&self, point: &Point) -> Option<u8> {
        if point.x < 0
            || point.y < 0
            || point.x >= self.width as i32
            || point.y >= self.height as i32
        {
            return None;
        }
        Some(self.board[point.y as usize][point.x as usize])
    }

    fn neighbours(&self, point: &Point) -> Vec<Point> {
        let mut neighbours = Vec::new();
        for direction in Direction::iter() {
            let neighbour = point.move_to(&direction);
            if let Some(c) = self.get(&neighbour) {
                neighbours.push(neighbour);
            }
        }
        neighbours
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let board = Board::new(input);
    let mut candidates = vec![];
    for i in 0..board.height {
        for j in 0..board.width {
            let point = Point {
                x: j as i32,
                y: i as i32,
            };
            if let Some(0) = board.get(&point) {
                candidates.push(point);
            }
        }
    }
    let res = candidates.iter().fold(0, |acc, p| acc + score(&board, p));
    Some(res)
}

fn score(board: &Board, start: &Point) -> u32 {
    let mut visited = HashSet::<Point>::new();
    let mut d = VecDeque::new();

    d.push_back((*start, 0));
    visited.insert(*start);
    let target = 9;
    let mut score = 0;

    while !d.is_empty() {
        let (p, v) = d.pop_front().unwrap();
        if v == target {
            score += 1;
            continue;
        }

        for neighbour in board.neighbours(&p) {
            if visited.contains(&neighbour) {
                continue;
            }
            match board.get(&neighbour) {
                Some(height) if height == v + 1 => {
                    d.push_back((neighbour, v + 1));
                    visited.insert(neighbour);
                }
                _ => continue,
            }
        }
    }
    score
}

pub fn part_two(input: &str) -> Option<u32> {
    let board = Board::new(input);
    let mut candidates = vec![];
    for i in 0..board.height {
        for j in 0..board.width {
            let point = Point {
                x: j as i32,
                y: i as i32,
            };
            if let Some(0) = board.get(&point) {
                candidates.push(point);
            }
        }
    }
    let res = candidates.iter().fold(0, |acc, p| acc + rating(&board, p));
    Some(res)
}

fn rating(board: &Board, start: &Point) -> u32 {
    let mut d = VecDeque::new();

    d.push_back((*start, 0));
    let target = 9;
    let mut score = 0;

    while !d.is_empty() {
        let (p, v) = d.pop_front().unwrap();
        if v == target {
            score += 1;
            continue;
        }

        for neighbour in board.neighbours(&p) {
            match board.get(&neighbour) {
                Some(height) if height == v + 1 => {
                    d.push_back((neighbour, v + 1));
                }
                _ => continue,
            }
        }
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
