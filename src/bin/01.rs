use itertools::Itertools;
use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut first, mut second) = parse_input(input);

    first.sort();
    second.sort();

    let diff = first
        .iter()
        .zip(second)
        .fold(0i64, |acc, (a, b)| acc + ((*a as i64) - b as i64).abs());
    Some(diff as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (first, second) = parse_input(input);

    let mut counter: HashMap<u32, u32> = HashMap::new();
    second.iter().for_each(|&x| {
        *counter.entry(x).or_insert(0) += 1;
    });

    let similarity = first
        .iter()
        .fold(0u32, |acc, x| acc + x * counter.get(&x).unwrap_or(&0));

    Some(similarity)
}

fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut first: Vec<u32> = Vec::new();
    let mut second: Vec<u32> = Vec::new();

    input.lines().for_each(|line| {
        if let Some((a, b)) = line.split_whitespace().collect_tuple() {
            first.push(a.parse().unwrap());
            second.push(b.parse().unwrap());
        }
    });
    (first, second)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
