use itertools::Itertools;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let (graph, mut updates) = parse_input(input);
    let result = updates.iter().filter(|&update| {
        let mut unsorted = update.clone();
        unsorted.sort_by(|a, b| {
            if graph.get(a).map(|set| set.contains(b)).unwrap_or(false) {
                std::cmp::Ordering::Less
            } else if graph.get(b).map(|set| set.contains(a)).unwrap_or(false) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });
        unsorted == *update
    }).fold(0, |acc, update| {
        update[update.len() / 2] + acc
    });
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (graph, mut updates) = parse_input(input);
    let mut result = 0;
    updates.iter().for_each(|update| {
        let mut unsorted = update.clone();
        unsorted.sort_by(|a, b| {
            if graph.get(a).map(|set| set.contains(b)).unwrap_or(false) {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        });
        if unsorted != *update {
            result += unsorted[unsorted.len() / 2];
        }
    });
    Some(result)
}

fn parse_input(input: &str) -> (HashMap<u32, HashSet<u32>>, Vec<Vec<u32>>) {
    let mut lines = &mut input.lines();
    let mut graph = HashMap::new();
    let mut updates = Vec::new();
    lines.map_while(|line| {
        line.split_once("|").and_then(|(a, b)| {
            let a = a.trim().parse().unwrap();
            let b = b.trim().parse().unwrap();
            Some((a, b))
        })
    }).for_each(|(a, b)| {
        graph.entry(a).or_insert(HashSet::new()).insert(b);
    });
    lines.map(|line| {
        line.split(",").map(|x| x.trim().parse().unwrap()).collect()
    }).for_each(|update| {
        updates.push(update);
    });

    (graph, updates)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
