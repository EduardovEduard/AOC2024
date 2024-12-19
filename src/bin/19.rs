use std::collections::HashMap;

advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<u32> {
    let (towels, designs) = parse_input(input);
    let mut map = HashMap::new();
    towels.iter().for_each(|towel| {
        map.entry(towel.chars().nth(0).unwrap())
            .or_insert(vec![])
            .push(towel.clone());
    });
    let result = designs.iter().filter(|design| {
        build(design, 0, &map)
    }).count();
    Some(result as u32)
}

fn build(target: &str, target_i: usize, candidates: &HashMap<char, Vec<String>>) -> bool {
    if target_i > target.len() {
        return false
    }
    if target_i == target.len() {
        return true;
    }

    let next = target.chars().nth(target_i).unwrap();
    let mut result = false;
    if let Some(candidates_list) = candidates.get(&next) {
        for candidate in candidates_list {
            if candidate.len() > target.len() - target_i {
                continue;
            }
            if target[target_i..target_i + candidate.len()] == *candidate.as_str() {
                result = result || build(target, target_i + candidate.len(), candidates)
            }
        }
    }
    result
}

pub fn part_two(input: &str) -> Option<u64> {
    let (towels, designs) = parse_input(input);
    let mut map = HashMap::new();
    towels.iter().for_each(|towel| {
        map.entry(towel.chars().nth(0).unwrap())
            .or_insert(vec![])
            .push(towel.clone());
    });
    let result = designs.iter().map(|design| {
        let mut cache = HashMap::new();
        build_count(design, 0, &map, &mut cache)
    }).sum();
    Some(result)
}

fn build_count(target: &str, target_i: usize, candidates: &HashMap<char, Vec<String>>, cache: &mut HashMap<usize, u64>) -> u64 {
    if target_i > target.len() {
        return 0;
    }
    if target_i == target.len() {
        return 1;
    }
    if let Some(&res) = cache.get(&target_i) {
        return res;
    }
    let next = target.chars().nth(target_i).unwrap();
    let mut result = 0;
    if let Some(candidates_list) = candidates.get(&next) {
        for candidate in candidates_list {
            if candidate.len() > target.len() - target_i {
                continue;
            }
            if target[target_i..target_i + candidate.len()] == *candidate.as_str() {
                result += build_count(target, target_i + candidate.len(), candidates, cache)
            }
        }
    }
    cache.insert(target_i, result);
    result
}

fn parse_input(input: &str) -> (Vec<String>, Vec<String>) {
    input.split_once("\n\n").map(|(towels, designs)| {
        (
            towels.split(", ").map(|l| l.to_string()).collect(),
            designs.lines().map(|l| l.to_string()).collect()
        )
    }).unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
