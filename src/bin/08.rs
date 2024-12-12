use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

struct Board {
    board: Vec<Vec<char>>,
    width: usize,
    height: usize,
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

    fn get(&self, point: &Point) -> Option<char> {
        if point.x < 0
            || point.y < 0
            || point.x >= self.width as i32
            || point.y >= self.height as i32
        {
            return None;
        }
        Some(self.board[point.y as usize][point.x as usize])
    }

    fn set(&mut self, point: &Point, char: char) {
        self.board[point.y as usize][point.x as usize] = char;
    }

    fn print(&self) {
        for row in self.board.iter() {
            println!("{}", row.iter().collect::<String>());
        }
    }
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn move_to(&self, dx: i32, dy: i32) -> Self {
        Self {
            x: self.x + dx,
            y: self.y + dy,
        }
    }

    fn diff(&self, other: &Point) -> (i32, i32) {
        (self.x - other.x, self.y - other.y)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut board = Board::new(input);
    let mut antinodes = HashSet::<Point>::new();
    let mut node_groups = HashMap::new();
    for i in 0..board.height {
        for j in 0..board.width {
            let p = Point::new(j as i32, i as i32);
            let c = board.get(&p).unwrap();
            if c == '.' {
                continue;
            }
            node_groups.entry(c).or_insert_with(|| vec![]).push(p);
        }
    }

    for (_, nodes) in node_groups.iter() {
        for i in 0..nodes.len() {
            for j in i + 1..nodes.len() {
                let left = nodes[i];
                let right = nodes[j];
                let diff = left.diff(&right);
                let mut next_left = left.move_to(diff.0, diff.1);
                if let Some(p) = board.get(&next_left) {
                    antinodes.insert(next_left);
                    next_left = next_left.move_to(diff.0, diff.1);
                }
                let mut next_right = right.move_to(-diff.0, -diff.1);
                if let Some(p) = board.get(&next_right) {
                    antinodes.insert(next_right);
                    next_right = next_right.move_to(-diff.0, -diff.1);
                }
            }
        }
    }

    Some(antinodes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut board = Board::new(input);
    let mut antinodes = HashSet::<Point>::new();
    let mut node_groups = HashMap::new();

    for i in 0..board.height {
        for j in 0..board.width {
            let p = Point::new(j as i32, i as i32);
            let c = board.get(&p).unwrap();
            if c == '.' {
                continue;
            }
            node_groups.entry(c).or_insert_with(|| vec![]).push(p);
        }
    }

    for (_, nodes) in node_groups.iter() {
        for i in 0..nodes.len() {
            for j in i + 1..nodes.len() {
                let left = nodes[i];
                let right = nodes[j];
                let diff = left.diff(&right);

                antinodes.insert(left);
                antinodes.insert(right);

                let mut next_left = left.move_to(diff.0, diff.1);
                while let Some(p) = board.get(&next_left) {
                    antinodes.insert(next_left);
                    next_left = next_left.move_to(diff.0, diff.1);
                }
                let mut next_right = right.move_to(-diff.0, -diff.1);
                while let Some(p) = board.get(&next_right) {
                    antinodes.insert(next_right);
                    next_right = next_right.move_to(-diff.0, -diff.1);
                }
            }
        }
    }

    Some(antinodes.len() as u32)
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
