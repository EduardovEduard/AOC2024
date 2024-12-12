use itertools::Itertools;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

advent_of_code::solution!(4);

#[derive(Debug, Eq, PartialEq, Copy, Clone, EnumIter)]
enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn move_to(&self, dir: &Direction) -> Self {
        match dir {
            Direction::N => Point {
                x: self.x,
                y: self.y - 1,
            },
            Direction::NE => Point {
                x: self.x + 1,
                y: self.y - 1,
            },
            Direction::E => Point {
                x: self.x + 1,
                y: self.y,
            },
            Direction::SE => Point {
                x: self.x + 1,
                y: self.y + 1,
            },
            Direction::S => Point {
                x: self.x,
                y: self.y + 1,
            },
            Direction::SW => Point {
                x: self.x - 1,
                y: self.y + 1,
            },
            Direction::W => Point {
                x: self.x - 1,
                y: self.y,
            },
            Direction::NW => Point {
                x: self.x - 1,
                y: self.y - 1,
            },
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
    X,
    M,
    A,
    S,
    Other(char),
}

impl Char {
    fn from_char(ch: char) -> Self {
        match ch {
            'X' => Char::X,
            'M' => Char::M,
            'A' => Char::A,
            'S' => Char::S,
            c => Char::Other(c),
        }
    }

    fn char(&self) -> char {
        match self {
            Char::X => 'X',
            Char::M => 'M',
            Char::A => 'A',
            Char::S => 'S',
            Char::Other(c) => *c,
        }
    }

    fn next(&self) -> Option<Self> {
        match self {
            Char::X => Some(Char::M),
            Char::M => Some(Char::A),
            Char::A => Some(Char::S),
            Char::S => None,
            Char::Other(c) => None,
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
        if point.x < 0
            || point.y < 0
            || point.x >= self.width as i32
            || point.y >= self.height as i32
        {
            return None;
        }
        Some(Char::from_char(
            self.board[point.y as usize][point.x as usize],
        ))
    }

    fn neighbours_of(&self, point: &Point, char: &Char) -> Vec<Point> {
        let mut neighbours = Vec::new();
        for direction in Direction::iter() {
            let neighbour = point.move_to(&direction);
            if let Some(c) = self.get(&neighbour) {
                if &c == char {
                    neighbours.push(neighbour);
                }
            }
        }
        neighbours
    }

    fn find_all(&self, char: &Char) -> Vec<Point> {
        let mut result = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let p = Point::new(x as i32, y as i32);
                if self.get(&p) == Some(*char) {
                    result.push(p);
                }
            }
        }
        result
    }

    fn find_xmas(&self, mut point: Point, dir: &Direction) -> Option<()> {
        let mut expected = Char::X;

        while self.get(&point)? != Char::S && self.get(&point)? == expected {
            expected = expected.next()?;
            point = point.move_to(&dir);
        }

        if self.get(&point)? == expected {
            Some(())
        } else {
            None
        }
    }

    fn has_x_mas(&self, point: &Point) -> bool {
        let dirs = vec![Direction::NW, Direction::NE, Direction::SE, Direction::SW];
        let mut candidates = vec![Char::M, Char::M, Char::S, Char::S];
        for _ in 0..4 {
            let matched = dirs
                .iter()
                .zip(&candidates)
                .all(|(dir, &expected)| self.get(&point.move_to(dir)) == Some(expected));

            if matched {
                return true;
            }

            candidates.rotate_left(1);
        }

        false
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let board = Board::new(input);
    let starts = board.find_all(&Char::X);

    let mut result = 0;
    for start in starts {
        for dir in Direction::iter() {
            match board.find_xmas(start, &dir) {
                Some(_) => result += 1,
                None => {}
            }
        }
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let board = Board::new(input);
    let starts = board.find_all(&Char::A);
    let mut result = 0;
    for start in starts {
        if board.has_x_mas(&start) {
            result += 1
        }
    }

    Some(result)
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
