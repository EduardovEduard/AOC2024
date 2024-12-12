use std::collections::{HashMap, HashSet, VecDeque};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

advent_of_code::solution!(12);

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone, EnumIter)]
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
}

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
        if point.x < 0 || point.y < 0 || point.x >= self.width as i32 || point.y >= self.height as i32 {
            return None;
        }
        Some(self.board[point.y as usize][point.x as usize])
    }

    fn set(&mut self, point: &Point, ch: char) {
        self.board[point.y as usize][point.x as usize] = ch;
    }

    fn neighbours_of(&self, point: &Point, ch: char) -> Vec<(Point, Direction)> {
        let mut neighbours = Vec::new();
        for direction in Direction::iter() {
            let neighbour = point.move_to(&direction);
            if let Some(c) = self.get(&neighbour) {
                if c == ch {
                    neighbours.push((neighbour, direction));
                }
            }
        }
        neighbours
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

    fn move_to(&self, dir: &Direction) -> Self {
        match dir {
            Direction::N => Point { x: self.x, y: self.y - 1 },
            Direction::E => Point { x: self.x + 1, y: self.y },
            Direction::S => Point { x: self.x, y: self.y + 1 },
            Direction::W => Point { x: self.x - 1, y: self.y },
        }
    }

    fn neighbours(&self) -> Vec<Point> {
        let mut neighbours = Vec::new();
        for direction in Direction::iter() {
            let neighbour = self.move_to(&direction);
            neighbours.push(neighbour);
        }
        neighbours
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let board = Board::new(input);
    let mut visited_all = HashSet::<Point>::new();
    let mut result = 0;

    for y in 0..board.height {
        for x in 0..board.width {
            let point = Point::new(x as i32, y as i32);
            let ch = board.get(&point).unwrap();
            if visited_all.contains(&point) {
                continue;
            }
            let (visited, area, perimeter) = calc_area_perimeter(&board, &point, ch);
            visited_all.extend(visited);
            result += area * perimeter;
        }
    }

    Some(result)
}

fn calc_area_perimeter(board: &Board, point: &Point, ch: char) -> (HashSet<Point>, u32, u32) {
    let mut dq = VecDeque::new();
    let mut visited = HashSet::new();
    dq.push_back(*point);
    visited.insert(*point);
    let mut area = 0;
    while !dq.is_empty() {
        let p = dq.pop_front().unwrap();
        area += 1;
        for (neighbour, _) in board.neighbours_of(&p, ch) {
            if visited.contains(&neighbour) {
                continue;
            }
            if board.get(&neighbour).unwrap() == ch {
                dq.push_back(neighbour);
                visited.insert(neighbour);
            }
        }
    }

    let perimeter = visited.iter().map(|p| {
        let count= p.neighbours().iter().filter(|&neighbour| {
            board.get(neighbour) != Some(ch)
        }).count() as u32;
        count
    }).sum();

    (visited, area, perimeter)
}

pub fn part_two(input: &str) -> Option<u32> {
    let board = Board::new(input);
    let mut visited_all = HashSet::<Point>::new();
    let mut result = 0;

    for y in 0..board.height {
        for x in 0..board.width {
            let point = Point::new(x as i32, y as i32);
            let ch = board.get(&point).unwrap();
            if visited_all.contains(&point) {
                continue;
            }
            let (visited, area, sides) = calc_area_sides(&board, &point, ch);
            visited_all.extend(visited);
            result += area * sides;
        }
    }
    Some(result)
}

fn calc_area_sides(board: &Board, point: &Point, ch: char) -> (HashSet<Point>, u32, u32) {
    let mut dq = VecDeque::new();
    let mut visited = HashSet::new();
    dq.push_back(*point);
    visited.insert(*point);
    let mut area = 0;
    while !dq.is_empty() {
        let p = dq.pop_front().unwrap();
        area += 1;
        for (neighbour, _) in board.neighbours_of(&p, ch) {
            if visited.contains(&neighbour) {
                continue;
            }
            if board.get(&neighbour).unwrap() == ch {
                dq.push_back(neighbour);
                visited.insert(neighbour);
            }
        }
    }

    let mut point_by_sides = HashMap::new();
    visited.iter().for_each(|&p| {
        let neighbours = p.neighbours();
        point_by_sides.insert(p, HashSet::new());
        neighbours.iter().zip(Direction::iter()).for_each(|(neighbour, dir)| {
            if board.get(neighbour) != Some(ch) {
                point_by_sides.get_mut(&p).unwrap().insert(dir);
            }
        });
    });

    let empty = HashSet::<Direction>::new();
    Direction::iter().for_each(|dir| {
        let mut visited_current_direction = HashSet::new();
        visited.iter().for_each(|&p| {
           if let Some(directions) = point_by_sides.get(&p) {
               if visited_current_direction.contains(&p) {
                   return;
               }
               if !directions.contains(&dir) {
                   return;
               }
               visited_current_direction.insert(p);

               let steps = choose_steps(&dir);
               steps.iter().for_each(|step_direction| {
                   let mut current = p.clone();
                   while let Some(left_ch) = board.get(&current.move_to(&step_direction)) {
                       current = current.move_to(&step_direction);
                       if ch != left_ch || !point_by_sides.get(&current).unwrap_or(&empty).contains(&dir) {
                           break;
                       }
                       visited_current_direction.insert(current);
                       point_by_sides.get_mut(&current).unwrap().remove(&dir);
                   }
               });
           }
        });
    });
    let sides = point_by_sides.iter().map(|(_, set)| set.len() as u32).sum();
    (visited, area, sides)
}

fn choose_steps(dir: &Direction) -> Vec<Direction> {
    match dir {
        Direction::N | Direction::S => vec![Direction::W, Direction::E],
        Direction::E | Direction::W => vec![Direction::N, Direction::S],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
