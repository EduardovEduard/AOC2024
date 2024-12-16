use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::io::empty;
use itertools::Itertools;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

advent_of_code::solution!(16);

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
enum CellType {
    Wall,
    Start,
    Finish,
    Empty,
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Cell {
    cell_type: CellType,
    point: Point,
}

impl Cell {
    fn new(cell_type: CellType, point: Point) -> Self {
        Self {
            cell_type,
            point,
        }
    }
}

struct Board {
    board: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point {
            x,
            y,
        }
    }

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

impl CellType {
    fn from_char(ch: char) -> Self {
        match ch {
            '#' => CellType::Wall,
            '.' => CellType::Empty,
            'S' => CellType::Start,
            'E' => CellType::Finish,
            _ => panic!("unexpected char: {}", ch)
        }
    }

    fn char(&self) -> char {
        match self {
            CellType::Wall => '#',
            CellType::Start => 'S',
            CellType::Finish => 'E',
            CellType::Empty => '.'
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone, EnumIter, Display)]
enum Direction {
    E,
    S,
    W,
    N,
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

    fn turn_left(&self) -> Direction {
        match self {
            Direction::N => Direction::W,
            Direction::W => Direction::S,
            Direction::S => Direction::E,
            Direction::E => Direction::N,
        }
    }

    fn opposite(&self) -> Direction {
        match self {
            Direction::N => Direction::S,
            Direction::S => Direction::N,
            Direction::W => Direction::E,
            Direction::E => Direction::W,
        }
    }
}

impl Board {
    fn new(input: &str) -> Self {
        let board: Vec<Vec<Cell>> = input.lines().enumerate().map(|(i, line)|
            line.chars().enumerate().map(|(j, ch)| {
                Cell::new(CellType::from_char(ch), Point::new(j as i32, i as i32))
            }).collect()
        ).collect();

        let width = board[0].len();
        let height = board.len();
        Self {
            board,
            width,
            height,
        }
    }

    fn get(&self, point: &Point) -> Option<Cell> {
        if point.x < 0
            || point.y < 0
            || point.x >= self.width as i32
            || point.y >= self.height as i32
        {
            return None;
        }
        Some(self.board[point.y as usize][point.x as usize])
    }

    fn set(&mut self, point: &Point, ch: char) {
        self.board[point.y as usize][point.x as usize] = Cell::new(CellType::from_char(ch), *point);
    }

    fn print(&self) {
        for row in self.board.iter() {
            println!("{}", row.iter().map(|c| c.cell_type.char()).collect::<String>());
        }
    }

    fn get_start(&self) -> Cell {
        self.board.iter()
            .flat_map(|row| row.iter())
            .find(|cell| {
                if let CellType::Start = cell.cell_type {
                    true
                } else {
                    false
                }
            }).unwrap().clone()
    }

    fn get_finish(&self) -> Cell {
        self.board.iter()
            .flat_map(|row| row.iter())
            .find(|cell| {
                if let CellType::Finish = cell.cell_type {
                    true
                } else {
                    false
                }
            }).unwrap().clone()
    }

    fn neighbours_of(&self, point: &Point, direction: &Direction) -> Vec<(Cell, Direction, u32)> {
        let directions = vec![
            (*direction, 1),
            (direction.turn_right(), 1001),
            (direction.turn_left(), 1001)
        ];
        let mut result = vec![];
        directions.iter().for_each(|&(dir, score)| {
            if let Some(cell) = self.get(&point.move_to(&dir)) {
                if matches!(cell.cell_type, CellType::Empty | CellType::Finish) {
                    result.push((cell, dir, score));
                }
            }
        });
       result
    }

    fn neighbours_of2(&self, point: &Point, direction: &Direction) -> Vec<(Cell, Direction, u32)> {
        let mut result = vec![];

        if let Some(cell) = self.get(&point.move_to(&direction)) {
            if matches!(cell.cell_type, CellType::Empty | CellType::Finish) {
                result.push((cell, *direction, 1));
            }
        }

        let turns = vec![
            direction.turn_right(),
            direction.turn_left()
        ];

        let current = self.get(&point).unwrap();
        turns.iter().for_each(|dir| {
            if let Some(cell) = self.get(&point.move_to(dir)) {
                if matches!(cell.cell_type, CellType::Empty | CellType::Finish) {
                    result.push((current, *dir, 1000));
                }
            }
        });
        result
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Entry {
    cell: Cell,
    direction: Direction,
    score: u32,
}

impl Entry {
    fn new(cell: Cell, direction: Direction, score: u32) -> Self {
        Self {
            cell,
            direction,
            score,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Entry2 {
    cell: Cell,
    direction: Direction,
    prev_cell: Cell,
    prev_direction: Direction,
    score: u32,
}

impl Entry2 {
    fn new(cell: Cell, direction: Direction, prev_cell: Cell, prev_direction: Direction, score: u32) -> Self {
        Self {
            cell,
            direction,
            prev_cell,
            prev_direction,
            score,
        }
    }
}
impl PartialOrd for Entry2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.score.cmp(&self.score))
    }
}

impl Ord for Entry2 {
    fn cmp(&self, other: &Self) -> Ordering {
        other.score.cmp(&self.score)
    }
}



impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.score.cmp(&self.score))
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        other.score.cmp(&self.score)
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    let board = Board::new(input);
    let start = board.get_start();
    let finish = board.get_finish();

    let mut dq = BinaryHeap::new();
    dq.push(Entry::new(start, Direction::E,0));

    let mut dist = HashMap::new();
    while !dq.is_empty() {
        let entry = dq.pop().unwrap();
        if dist.contains_key(&entry.cell.point) && dist.get(&entry.cell.point).unwrap() <= &entry.score {
            continue;
        }
        dist.insert(entry.cell.point, entry.score);
        if entry.cell.point == finish.point {
            continue;
        }
        let neighbours = board.neighbours_of(&entry.cell.point, &entry.direction);
        for (cell, dir, step_score) in neighbours {
            dq.push(Entry::new(cell, dir, entry.score + step_score));
        }
    }

    Some(dist.get(&finish.point).copied().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let board = Board::new(input);
    let start = board.get_start();
    let finish = board.get_finish();

    let mut dq = BinaryHeap::new();
    let prev_cell = Cell {
        cell_type: CellType::Empty,
        point: Point::new(-1, -1),
    };
    dq.push(Entry2::new(start, Direction::E, prev_cell, Direction::E, 0 ));

    let mut dist = HashMap::new();
    let mut prevs = HashMap::new();
    let mut min_dist = u32::MAX;
    prevs.insert((start.point, Direction::E), vec![]);

    while !dq.is_empty() {
        let entry = dq.pop().unwrap();
        let key = (entry.cell.point, entry.direction);
        if dist.contains_key(&key) && dist.get(&key).unwrap() < &entry.score {
            continue;
        }

        if entry.cell.point != start.point {
            if prevs.contains_key(&key) {
                if dist.get(&key).unwrap() == &entry.score {
                    prevs.entry(key)
                        .or_insert(vec![])
                        .push((entry.prev_cell, entry.prev_direction, entry.score))
                }
            } else {
                prevs.insert(key, vec![(entry.prev_cell, entry.prev_direction, entry.score)]);
            }
        }

        if dist.contains_key(&key) && dist.get(&key).unwrap() <= &entry.score {
            continue;
        }

        dist.insert(key, entry.score);
        if entry.cell.point == finish.point {
            min_dist = entry.score.min(min_dist);
            continue;
        }

        let neighbours = board.neighbours_of2(&entry.cell.point, &entry.direction);
        for (cell, dir, step_score) in neighbours {
            dq.push(Entry2::new(cell, dir, entry.cell, entry.direction, entry.score + step_score));
        }
    }

    let mut sources = VecDeque::new();
    sources.extend(Direction::iter().map(|dir| (finish.point, dir)).filter(|key| {
        dist.get(key).unwrap_or(&u32::MAX) == &min_dist
    }));

    let mut result = HashSet::new();
    result.insert(finish.point);
    let empty = vec![];
    for (key, value) in sources.iter() {
        let p = prevs.get(&(*key, *value)).or(Some(&empty)).unwrap();
    }
    while !sources.is_empty() {
        let cur = sources.pop_front().unwrap();
        let source = prevs.get(&cur).or(Some(&empty)).unwrap();
        source.iter().for_each(|(prev, direction, _)| {
            result.insert(prev.point);
            sources.push_back((prev.point, *direction));
        });
    }

    Some(result.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
