use std::fmt::{Display, Formatter, Pointer};
use std::ops::Mul;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

advent_of_code::solution!(15);

struct Board {
    board: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
}

struct ScaledBoard {
    board: Vec<Vec<ScaledCell>>,
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

impl Mul<u32> for Point {
    type Output = Self;

    fn mul(self, rhs: u32) -> Self::Output {
        Self {
            x: self.x * rhs as i32,
            y: self.y * rhs as i32,
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
enum Cell {
    Wall(Point),
    Box(Point),
    Robot(Point),
    Empty(Point),
}

impl Cell {
    fn from_char(ch: char, point: &Point) -> Self {
        match ch {
            '#' => Cell::Wall(*point),
            '.' => Cell::Empty(*point),
            'O' => Cell::Box(*point),
            '@' => Cell::Robot(*point),
            _ => panic!("unexpected char: {}", ch)
        }
    }

    fn char(&self) -> char {
        match self {
            Cell::Wall(_) => '#',
            Cell::Box(_) => 'O',
            Cell::Robot(_) => '@',
            Cell::Empty(_) => '.'
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, Display)]
enum ScaledCell {
    Wall(Point),
    BoxLeft(Point),
    BoxRight(Point),
    Robot(Point),
    Empty(Point),
}

impl ScaledCell {
    fn from_char(ch: char, point: &Point) -> Self {
        match ch {
            '#' => ScaledCell::Wall(*point),
            '.' => ScaledCell::Empty(*point),
            '[' => ScaledCell::BoxLeft(*point),
            ']' => ScaledCell::BoxRight(*point),
            '@' => ScaledCell::Robot(*point),
            _ => panic!("unexpected char: {}", ch)
        }
    }

    fn other_box_edge(&self) -> Point {
        match self {
            ScaledCell::BoxLeft(p) => p.move_to(&Direction::E),
            ScaledCell::BoxRight(p) => p.move_to(&Direction::W),
            _ => panic!("not a box")
        }
    }

    fn char(&self) -> char {
        match self {
            ScaledCell::Wall(_) => '#',
            ScaledCell::BoxLeft(_) => '[',
            ScaledCell::BoxRight(_) => ']',
            ScaledCell::Robot(_) => '@',
            ScaledCell::Empty(_) => '.'
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
    fn from_char(ch: char) -> Option<Self> {
        match ch {
            '<' => Some(Direction::W),
            '>' => Some(Direction::E),
            '^' => Some(Direction::N),
            'v' => Some(Direction::S),
            _ => None
        }
    }

    fn is_horizontal(&self) -> bool {
        match self {
            Direction::E | Direction::W => true,
            _ => false
        }
    }
}

impl Board {
    fn new(input: &str) -> Self {
        let board: Vec<Vec<Cell>> = input.lines().enumerate().map(|(i, line)|
            line.chars().enumerate().map(|(j, ch)| {
                Cell::from_char(ch, &Point::new(j as i32, i as i32))
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
        self.board[point.y as usize][point.x as usize] = Cell::from_char(ch, point);
    }

    fn print(&self) {
        for row in self.board.iter() {
            println!("{}", row.iter().map(|c| c.char()).collect::<String>());
        }
    }

    fn try_move(&mut self, cell: Cell, direction: Direction) -> bool {
        match cell {
            Cell::Wall(_) => false,
            Cell::Empty(_) => true,
            Cell::Robot(p) | Cell::Box(p) => {
                let new_p = p.move_to(&direction);
                match self.get(&new_p) {
                    Some(next_cell) if self.try_move(next_cell, direction) => {
                        self.swap(p.clone(), new_p);
                        true
                    }
                    _ => false
                }
            }
        }
    }

    fn swap(&mut self, a: Point, b: Point) {
        let a_cell = self.get(&a).unwrap();
        let b_cell = self.get(&b).unwrap();
        self.set(&b, a_cell.char());
        self.set(&a, b_cell.char())
    }

    fn get_robot(&self) -> Cell {
        self.board.iter()
            .flat_map(|row| row.iter())
            .find(|cell| {
                if let Cell::Robot(_) = cell {
                    true
                } else {
                    false
                }
            }).unwrap().clone()
    }

    fn as_scaled_board(&self) -> ScaledBoard {
        let mut scaled_board = vec![];
        for (i, row) in self.board.iter().enumerate() {
            let mut scaled_row = vec![];
            for (j, cell) in row.iter().enumerate() {
                match cell {
                    Cell::Wall(_) => {
                        scaled_row.push(ScaledCell::Wall(Point::new(j as i32 * 2, i as i32)));
                        scaled_row.push(ScaledCell::Wall(Point::new(j as i32 * 2 + 1, i as i32)));
                    }
                    Cell::Box(_) => {
                        scaled_row.push(ScaledCell::BoxLeft(Point::new(j as i32 * 2, i as i32)));
                        scaled_row.push(ScaledCell::BoxRight(Point::new(j as i32 * 2 + 1, i as i32)));
                    }
                    Cell::Robot(_) => {
                        scaled_row.push(ScaledCell::Robot(Point::new(j as i32 * 2, i as i32)));
                        scaled_row.push(ScaledCell::Empty(Point::new(j as i32 * 2 + 1, i as i32)));
                    }
                    Cell::Empty(_) => {
                        scaled_row.push(ScaledCell::Empty(Point::new(j as i32 * 2, i as i32)));
                        scaled_row.push(ScaledCell::Empty(Point::new(j as i32 * 2 + 1, i as i32)));
                    }
                }
            }
            scaled_board.push(scaled_row);
        }
        ScaledBoard {
            board: scaled_board,
            width: self.width * 2,
            height: self.height,
        }
    }
}


pub fn part_one(input: &str) -> Option<u32> {
    let (mut board, directions) = parse_input(input);
    board.print();

    let mut robot = board.get_robot();
    for direction in directions {
        if board.try_move(robot, direction) {
            if let Cell::Robot(p) = robot {
                robot = board.get(&p.move_to(&direction)).unwrap();
            }
        }
    }
    board.print();
    let mut gps = 0;
    for row in board.board.iter() {
        for cell in row.iter() {
            if let Cell::Box(p) = cell {
                gps += 100 * p.y + p.x;
            }
        }
    }
    Some(gps as u32)
}

impl ScaledBoard {
    fn new(input: &str) -> Self {
        let board: Vec<Vec<ScaledCell>> = input.lines().enumerate().map(|(i, line)|
            line.chars().enumerate().map(|(j, ch)| {
                ScaledCell::from_char(ch, &Point::new(j as i32, i as i32))
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

    fn print(&self) {
        for row in self.board.iter() {
            println!("{}", row.iter().map(|c| c.char()).collect::<String>());
        }
    }

    fn do_move(&mut self, cell: ScaledCell, direction: &Direction) {
        match cell {
            ScaledCell::Robot(p) => self.do_move_(p, &direction),
            ScaledCell::BoxLeft(p) | ScaledCell::BoxRight(p) if direction.is_horizontal() => self.do_move_(p, &direction),
            box_cell @ (ScaledCell::BoxLeft(p) | ScaledCell::BoxRight(p)) => {
                let other_edge= box_cell.other_box_edge();
                self.do_move_(p, &direction);
                self.do_move_(other_edge, &direction);
            }
            _ => {}
        }
    }

    fn try_move(&mut self, cell: ScaledCell, direction: &Direction) -> bool {
        match cell {
            ScaledCell::Wall(_) => false,
            ScaledCell::Empty(_) => true,
            ScaledCell::Robot(p) => self.try_pre_move(p, &direction),
            ScaledCell::BoxLeft(p) | ScaledCell::BoxRight(p) if direction.is_horizontal() => self.try_pre_move(p, &direction),
            box_cell @ (ScaledCell::BoxLeft(p) | ScaledCell::BoxRight(p)) => {
                let other_edge= box_cell.other_box_edge();
                self.try_pre_move(p, &direction) && self.try_pre_move(other_edge, &direction)
            }
        }
    }

    fn try_pre_move(&mut self, p: Point, direction: &Direction) -> bool {
        let new_p = p.move_to(&direction);
        match self.get(&new_p) {
            Some(next_cell) if self.try_move(next_cell, direction) => {
                println!("Can move {} to {}, ({}, {})", next_cell, direction, p.x, p.y);
                true
            }
            _ => false
        }
    }

    fn do_move_(&mut self, p: Point, direction: &Direction) {
        let new_p = p.move_to(&direction);
        let next_cell = self.get(&new_p).unwrap();
        self.do_move(next_cell, direction);
        self.swap(p.clone(), new_p);
    }

    fn swap(&mut self, a: Point, b: Point) {
        let a_cell = self.get(&a).unwrap();
        let b_cell = self.get(&b).unwrap();
        self.set(&b, a_cell.char());
        self.set(&a, b_cell.char())
    }

    fn get(&self, point: &Point) -> Option<ScaledCell> {
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
        self.board[point.y as usize][point.x as usize] = ScaledCell::from_char(ch, point);
    }

    fn get_robot(&self) -> ScaledCell {
        self.board.iter()
            .flat_map(|row| row.iter())
            .find(|cell| {
                if let ScaledCell::Robot(_) = cell {
                    true
                } else {
                    false
                }
            }).unwrap().clone()
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut board, directions) = parse_input(input);

    let mut scaled_board = board.as_scaled_board();
    scaled_board.print();

    let mut robot = scaled_board.get_robot();
    println!("{:?}", robot);
    for direction in directions {
        if scaled_board.try_move(robot, &direction) {
            scaled_board.do_move(robot, &direction);
            if let ScaledCell::Robot(p) = robot {
                robot = scaled_board.get(&p.move_to(&direction)).unwrap();
            }
        }
    }

    scaled_board.print();
    let mut gps = 0;
    for row in scaled_board.board.iter() {
        for cell in row.iter() {
            if let ScaledCell::BoxLeft(p) = cell {
                gps += 100 * p.y + p.x;
            }
        }
    }
    Some(gps as u32)
}

fn parse_input(input: &str) -> (Board, Vec<Direction>) {
    let (board, directions) = input.split_once("\n\n").unwrap();
    (Board::new(board), directions.chars()
        .map(Direction::from_char)
        .filter(|o| o.is_some())
        .map(|o| o.unwrap())
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));

        assert_eq!(result, Some(9021));
    }
}
