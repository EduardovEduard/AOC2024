use itertools::Itertools;
use std::collections::BTreeSet;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let mut disk = parse_disk(input);
    let mut i = 0;
    let mut j = disk.len() - 1;
    while i < j {
        while disk[j] == None {
            j -= 1;
        }
        while disk[i] != None {
            i += 1;
        }
        if i < j {
            disk.swap(i, j);
        }
    }
    Some(checksum(&disk))
}

fn checksum(disk: &Vec<Option<u64>>) -> u64 {
    disk.iter()
        .enumerate()
        .fold(0, |acc, (i, &x)|
            x.map(|x| i as u64 * x + acc).unwrap_or(acc)
        )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut disk = parse_disk(input);
    let mut blocks_set = BTreeSet::<(usize, usize)>::new();
    let mut file_map: Vec<(usize, usize)> = vec![];

    let mut i = 0;
    let mut j = 1;
    let mut file = disk[i] != None;
    let mut next_id = 0;
    while i < disk.len() {
        while j < disk.len()
            && ((file && disk[j] != None && disk[j].unwrap() == next_id)
            || (!file && disk[j] == None)) {
            j += 1
        }
        if file {
            file_map.push((i, j));
        } else {
            blocks_set.insert((i, j));
        }
        if j < disk.len() && disk[j] != None {
            next_id += 1;
            file = true;
        } else {
            file = false;
        }
        i = j;
    }

    let mut file_id = file_map.len() - 1;
    while file_id != usize::MAX {
        let (file_i, file_j) = file_map[file_id];
        let file_size = file_j - file_i;

        let mut iter = blocks_set.iter();

        while let Some(&(block_i, block_j)) = iter.next() {
            let block_size = block_j - block_i;
            if block_i > file_i {
                break;
            }
            if block_size < file_size {
                continue
            }
            drop(iter);

            blocks_set.remove(&(block_i, block_j));
            if block_size > file_size {
                blocks_set.insert((block_i + file_size, block_j));
            }
            disk[block_i..block_i + file_size].copy_from_slice(vec![Some(file_id as u64); file_size].as_slice());
            disk[file_i..file_j].copy_from_slice(vec![None; file_size].as_slice());
            break;
        }

        file_id = file_id.wrapping_sub(1);
    }
    Some(checksum(&disk))
}

fn parse_disk(input: &str) -> Vec<Option<u64>> {
    let compressed: Vec<u32> = input.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let mut disk: Vec<Option<u64>> = vec![];
    let mut id = 0u64;
    for (i, c) in compressed.iter().enumerate() {
        if i % 2 == 0 {
            disk.append(&mut vec![Some(id); *c as usize]);
            id += 1;
        } else {
            disk.append(&mut vec![None; *c as usize]);
        }
    }
    disk
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
