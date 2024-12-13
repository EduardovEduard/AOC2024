use itertools::Itertools;
use regex::Regex;

advent_of_code::solution!(13);

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Move {
    x: i64,
    y: i64
}

pub fn part_one(input: &str) -> Option<u32> {
    let machines = parse_input(input);
    println!("{:?}", machines);
    let mut result = 0;
    machines.iter().for_each(|(a, b, prize)| {
        if let Some((A, B)) = solve(a, b, prize) {
            result += A * 3 + B;
        }
    });
    Some(result as u32)
}

fn solve(a: &Move, b: &Move, prize: &Move) -> Option<(i64, i64)> {
    let [xa, xb] = [a.x, b.x];
    let [ya, yb] = [a.y, b.y];
    let [xp, yp] = [prize.x, prize.y];

    let b_solved = (xa * yp - (xp * ya)) / (xa * yb - (xb * ya));
    let a_solved = (xp - (xb * b_solved)) / xa;

    if xp == a_solved * xa + b_solved * xb && yp == a_solved * ya + b_solved * yb {
        println!("Solution found for {:?} {:?} {:?}: A: {}, B: {}", a, b, prize, a_solved, b_solved);
        Some((a_solved, b_solved))
    } else {
        println!("No solution found for {:?} {:?} {:?}", a, b, prize);
        None
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let machines = parse_input(input);
    println!("{:?}", machines);
    let mut result = 0;
    machines.iter().for_each(|(a, b, prize)| {
        let new_prize = Move {
            x: prize.x + 10000000000000,
            y: prize.y + 10000000000000,
        };
        if let Some((A, B)) = solve(a, b, &new_prize) {
            result += A * 3 + B;
        }
    });
    Some(result as u64)
}

fn parse_input(input: &str) -> Vec<(Move, Move, Move)> {
    let button_reg = Regex::new(r"Button (\w): X\+(\d+), Y\+(\d+)").unwrap();
    let prize_reg = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
    let mut chunks = input.lines().chunks(4);
    let mut machines = vec![];
    for mut chunk in chunks.into_iter() {
        let (_, [_, ax, ay]) = button_reg.captures(chunk.next().unwrap()).unwrap().extract();
        let (_, [_, bx, by]) = button_reg.captures(chunk.next().unwrap()).unwrap().extract();
        let (_, [prize_x, prize_y]) = prize_reg.captures(chunk.next().unwrap()).unwrap().extract();
        machines.push((
            Move { x: ax.parse().unwrap(), y: ay.parse().unwrap() },
            Move { x: bx.parse().unwrap(), y: by.parse().unwrap() },
            Move { x: prize_x.parse().unwrap(), y: prize_y.parse().unwrap() },
        ));
    }
    machines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
