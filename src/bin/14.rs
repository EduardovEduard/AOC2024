use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

advent_of_code::solution!(14);

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Velocity {
    x: i32,
    y: i32,
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut robots = parse_input(input);
    let width = 101; // 101;
    let height = 103; //103;

    let mut board = vec![vec!['.'; width as usize]; height as usize];
    for robot in &robots {
        board[robot.0.y as usize][robot.0.x as usize] = '#';
    }

    for robot in &mut robots {
        robot.0.x = ((robot.0.x + robot.1.x * 100) % width + width) % width;
        robot.0.y = ((robot.0.y + robot.1.y * 100) % height + height) % height;
    }

    let mut board = vec![vec!['.'; width as usize]; height as usize];
    for robot in &robots {
        board[robot.0.y as usize][robot.0.x as usize] = '#';
    }

    let quad_w = width / 2;
    let quad_h = height / 2;

    let mut counts = vec![0, 0, 0, 0];
    for robot in &robots {
        if robot.0.x < quad_w && robot.0.y < quad_h {
            counts[0] += 1;
        } else if robot.0.x > quad_w && robot.0.y < quad_h {
            counts[1] += 1;
        } else if robot.0.x < quad_w && robot.0.y > quad_h {
            counts[2] += 1;
        } else if robot.0.x > quad_w && robot.0.y > quad_h {
            counts[3] += 1;
        }
    }

    Some(counts.iter().fold(1, |acc, x| acc * x))
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut robots = parse_input(input);
    let width = 101;
    let height = 103;

    for i in 1..10000000 {
        for robot in &mut robots {
            robot.0.x = ((robot.0.x + robot.1.x) % width + width) % width;
            robot.0.y = ((robot.0.y + robot.1.y) % height + height) % height;
        }

        if find_vertical(&mut robots, 10) {
            println!("Vertical found at {}", i);
            let mut boardr = vec![vec!['.'; width as usize]; height as usize];
            for robot in &robots {
                boardr[robot.0.y as usize][robot.0.x as usize] = '#';
            }
            for row in boardr {
                println!("{}", row.iter().collect::<String>());
            }
            return Some(i);
        }
    }

    None
}

fn find_vertical(robots: &mut Vec<(Point, Velocity)>, size: u32) -> bool {
    let mut by_x = HashMap::new();
    robots.iter().for_each(|(point, _)| {
        by_x.entry(point.x).or_insert(vec![]).push(point.y);
    });
    let mut count = 1;
    for (x, ys) in &mut by_x {
        ys.sort();
        let mut current = ys[0];
        for next in ys.iter().skip(1) {
            if (next - current).abs() == 1 {
                count += 1;
            } else {
                count = 1;
            }
            if count == size {
                return true;
            }
            current = *next;
        }
    }
    count == size
}

fn parse_input(input: &str) -> Vec<(Point, Velocity)> {
    let line_reg = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
    input
        .lines()
        .map(|line| {
            let (_, [px, py, vx, vy]) = line_reg.captures(line).unwrap().extract();
            (
                Point {
                    x: px.parse().unwrap(),
                    y: py.parse().unwrap(),
                },
                Velocity {
                    x: vx.parse().unwrap(),
                    y: vy.parse().unwrap(),
                },
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_find_vertical() {
        assert_eq!(
            find_vertical(
                &mut vec![
                    (Point { x: 10, y: 11 }, Velocity { x: 1, y: 1 }),
                    (Point { x: 10, y: 12 }, Velocity { x: 1, y: 1 }),
                    (Point { x: 10, y: 13 }, Velocity { x: 1, y: 1 }),
                    (Point { x: 10, y: 14 }, Velocity { x: 1, y: 1 }),
                    (Point { x: 10, y: 15 }, Velocity { x: 1, y: 1 }),
                ],
                5
            ),
            true
        );

        assert_eq!(
            find_vertical(
                &mut vec![
                    (Point { x: 8, y: 11 }, Velocity { x: 1, y: 1 }),
                    (Point { x: 7, y: 12 }, Velocity { x: 1, y: 1 }),
                    (Point { x: 10, y: 13 }, Velocity { x: 1, y: 1 }),
                    (Point { x: 10, y: 14 }, Velocity { x: 1, y: 1 }),
                    (Point { x: 10, y: 15 }, Velocity { x: 1, y: 1 }),
                    (Point { x: 10, y: 16 }, Velocity { x: 1, y: 1 }),
                    (Point { x: 10, y: 17 }, Velocity { x: 1, y: 1 }),
                    (Point { x: 10, y: 18 }, Velocity { x: 1, y: 1 }),
                    (Point { x: 10, y: 19 }, Velocity { x: 1, y: 1 }),
                    (Point { x: 10, y: 29 }, Velocity { x: 1, y: 1 }),
                    (Point { x: 10, y: 20 }, Velocity { x: 1, y: 1 }),
                    (Point { x: 11, y: 15 }, Velocity { x: 1, y: 1 }),
                    (Point { x: 11, y: 16 }, Velocity { x: 1, y: 1 }),
                ],
                8
            ),
            true
        );

        assert_eq!(
            find_vertical(
                &mut vec![
                    (Point { x: 10, y: 11 }, Velocity { x: 1, y: 1 }),
                    (Point { x: 10, y: 12 }, Velocity { x: 1, y: 1 }),
                    (Point { x: 10, y: 14 }, Velocity { x: 1, y: 1 }),
                    (Point { x: 10, y: 15 }, Velocity { x: 1, y: 1 }),
                    (Point { x: 10, y: 16 }, Velocity { x: 1, y: 1 }),
                    (Point { x: 10, y: 17 }, Velocity { x: 1, y: 1 }),
                    (Point { x: 10, y: 19 }, Velocity { x: 1, y: 1 }),
                    (Point { x: 10, y: 29 }, Velocity { x: 1, y: 1 }),
                ],
                4
            ),
            false
        );
    }
}
