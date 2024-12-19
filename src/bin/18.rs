use std::collections::{HashSet, VecDeque};
use itertools::Itertools;

advent_of_code::solution!(18);

pub fn part_one(input: &str) -> Option<u32> {
    let coords = parse_input(input);
    let size = 71;
    let mut board = vec![vec!['.'; size]; size];
    for (x, y) in coords[0..1024].iter() {
        board[*y as usize][*x as usize] = '#';
    }
    if let res@  Some(value) = bfs(&board) {
        return res;
    }
    None
}

pub fn part_two(input: &str) -> Option<String> {
    let coords = parse_input(input);
    let size = 71;
    let mut board = vec![vec!['.'; size]; size];
    for (i, &(x, y)) in coords.iter().enumerate() {
        board[y as usize][x as usize] = '#';

        if let Some(value) = bfs(&board) {
            continue;
        }

        return Some(format!("{},{}", x, y));
    }
    None
}

fn bfs(board: &Vec<Vec<char>>) -> Option<u32> {
    let start = (0, 0);
    let mut q = VecDeque::new();
    let size = board.len();
    q.push_back((start, 0));
    let mut visited = HashSet::new();
    while !q.is_empty() {
        let (cur, d) = q.pop_front().unwrap();
        if cur == (size as i32 - 1, size as i32 - 1) {
            return Some(d as u32);
        }
        if visited.contains(&cur) {
            continue;
        }
        visited.insert(cur);
        for (dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let new_x = cur.0 + dx;
            let new_y = cur.1 + dy;
            if new_x >= 0 && new_x < size as i32 && new_y >= 0 && new_y < size as i32 &&
                board[new_y as usize][new_x as usize] == '.' &&
                !visited.contains(&(new_x, new_y))
            {
                q.push_back(((new_x, new_y), d + 1));
            }
        }
    }
    None
}

pub fn parse_input(input: &str) -> Vec<(u32, u32)> {
    input.lines().map(|line| {
        let parts = line.split_once(",").unwrap();
        let a = parts.0.parse().unwrap();
        let b = parts.1.parse().unwrap();
        (a, b)
    }).collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(String::from("6,1")));
    }
}
