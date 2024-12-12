use std::collections::HashMap;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u32> {
    let mut stones = parse_input(input);
    for i in 0..25 {
        stones = blink(stones);
    }
    Some(stones.len() as u32)
}

fn blink(stones: Vec<u128>) -> Vec<u128> {
    let mut new_stones = vec![];
    stones.iter().for_each(|stone| match stone {
        0 => new_stones.push(1),
        stone if even(stone) => {
            let base = 10_u128.pow((stone.ilog10() + 1) / 2);
            new_stones.push(stone / base);
            new_stones.push(stone % base)
        }
        _ => new_stones.push(stone * 2024),
    });
    new_stones
}

fn even(v: &u128) -> bool {
    (v.ilog10() + 1) % 2 == 0
}

pub fn part_two(input: &str) -> Option<u128> {
    let mut stones = parse_input(input);
    let mut cache = HashMap::new();
    Some(
        stones
            .iter()
            .map(|&x| fast_blink(&mut cache, x, 75) as u128)
            .sum::<u128>(),
    )
}

fn fast_blink(cache: &mut HashMap<(u128, u32), u64>, stone: u128, blinks: u32) -> u64 {
    if blinks == 0 {
        return 1;
    }

    if cache.contains_key(&(stone, blinks)) {
        return cache[&(stone, blinks)];
    }

    let result = match stone {
        0 => fast_blink(cache, 1, blinks - 1),
        stone if even(&stone) => {
            let base = 10_u128.pow((stone.ilog10() + 1) / 2);
            let left = fast_blink(cache, stone / base, blinks - 1);
            let right = fast_blink(cache, stone % base, blinks - 1);
            left + right
        }
        _ => fast_blink(cache, stone * 2024, blinks - 1),
    };

    cache.insert((stone, blinks), result);
    result
}

fn parse_input(input: &str) -> Vec<u128> {
    input.split(" ").map(|x| x.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
