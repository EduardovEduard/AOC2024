use std::collections::{HashSet, VecDeque};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

advent_of_code::solution!(20);

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone, EnumIter)]
enum Direction {
    E,
    S,
    W,
    N,
}


#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn distance_to(&self, x: usize, y: usize) -> usize {
        (self.x as i32 - x as i32).abs() as usize +
            (self.y as i32 - y as i32).abs() as usize
    }

    fn move_to(&self, dir: &Direction) -> Self {
        match dir {
            Direction::N => Point {
                x: self.x,
                y: self.y.wrapping_sub(1),
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
                x: self.x.wrapping_sub(1),
                y: self.y,
            },
        }
    }
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
        if point.x >= self.width || point.y >= self.height {
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

    fn get_pos_of(&self, ch: char) -> Point {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.board[y][x] == ch {
                    return Point::new(x, y);
                }
            }
        }
        panic!("Could not find position of {}", ch);
    }

    fn print(&self) {
        for row in self.board.iter() {
            println!("{}", row.iter().collect::<String>());
        }
    }
}


pub fn part_one(input: &str) -> Option<u32> {
    let mut board = Board::new(input);
    let start = board.get_pos_of('S');
    let finish = board.get_pos_of('E');
    let mut dist = vec![vec![std::u32::MAX; board.width]; board.height];

    board.print();

    let distance= bfs(&board, &mut dist, start, finish).unwrap();
    reverse_update_dist(&mut dist, distance);

    let mut result = 0;
    for y in 0..board.height {
        for x in 0..board.width {
            let point = Point::new(x, y);
            let current_distance = dist[y][x];
            if current_distance != u32::MAX {
                for direction in Direction::iter() {
                    let wall_candidate = point.move_to(&direction);
                    if  Some('#') == board.get(&wall_candidate) {
                        let peek = wall_candidate.move_to(&direction);
                        if let Some('.' | 'E') = board.get(&peek) {
                            if dist[peek.y][peek.x] != u32::MAX {
                                let cheated_dist = dist[peek.y][peek.x];
                                let save = current_distance as i32 - cheated_dist as i32 - 2;
                                if save >= 100 {
                                    result += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut board = Board::new(input);
    let start = board.get_pos_of('S');
    let finish = board.get_pos_of('E');
    let mut dist = vec![vec![std::u32::MAX; board.width]; board.height];

    board.print();

    let distance = bfs(&board, &mut dist, start, finish).unwrap();
    reverse_update_dist(&mut dist, distance);

    let mut result = 0;
    let cheat_seconds = 20i32;
    let cheat_min_save = 100;
    for y in 0..board.height {
        for x in 0..board.width {
            let current_distance = dist[y][x];
            if current_distance != u32::MAX {
                for i in -cheat_seconds..=cheat_seconds {
                    for j in -cheat_seconds..=cheat_seconds {
                        let cheat_path_dist = i.abs() + j.abs();
                        if cheat_path_dist <= cheat_seconds {
                            let cheat_candidate = Point::new(x.wrapping_add_signed(i as isize), y.wrapping_add_signed(j as isize));
                            if let Some('E' | '.') = board.get(&cheat_candidate) {
                                let cheat_candidate_dist = dist[cheat_candidate.y][cheat_candidate.x];
                                if cheat_candidate_dist < current_distance {
                                    let cheat_save= current_distance.wrapping_sub(cheat_candidate_dist).wrapping_sub(cheat_path_dist as u32);
                                    if cheat_save >= cheat_min_save {
                                        result += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Some(result)
}

fn reverse_update_dist(mut dist: &mut Vec<Vec<u32>>, distance: u32) {
    for y in 0..dist.len() {
        let mut row = vec![];
        for x in 0..dist[0].len() {
            if dist[y][x] != u32::MAX {
                dist[y][x] = distance - dist[y][x];
            }
            row.push(dist[y][x]);
        }
    }
}

fn bfs(board: &Board, dist: &mut Vec<Vec<u32>>, start: Point, end: Point) -> Option<u32> {
    let mut q = VecDeque::new();
    q.push_back((start, 0));
    let mut visited = HashSet::new();
    visited.insert(start);

    while !q.is_empty() {
        let (cur, d) = q.pop_front().unwrap();
        dist[cur.y as usize][cur.x as usize] = d;
        if cur == end {
            return Some(d);
        }

        for direction in Direction::iter() {
            let new_point = cur.move_to(&direction);
            if visited.contains(&new_point) {
                continue;
            }

            if let Some(c) = board.get(&new_point) {
                if c == '.' || c == 'E' {
                    visited.insert(cur);
                    q.push_back((new_point, d + 1));
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(44));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(285));
    }
}
