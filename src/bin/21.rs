use std::collections::{HashMap, VecDeque};
use itertools::Itertools;

advent_of_code::solution!(21);

pub fn part_one(input: &str) -> Option<u32> {
    /*
    +---+---+---+
    | 7 | 8 | 9 |
    +---+---+---+
    | 4 | 5 | 6 |
    +---+---+---+
    | 1 | 2 | 3 |
    +---+---+---+
        | 0 | A |
        +---+---+
     */
    let keypad = vec![
        vec!['7', '8', '9'],
        vec!['4', '5', '6'],
        vec!['1', '2', '3'],
        vec!['X', '0', 'A'],
    ];
    let arrow_pad = vec![
        vec!['X', '^', 'A'],
        vec!['<', 'v', '>'],
    ];
    let keypad_moves = build_moves(&keypad, false);
    let arrow_pad_moves = build_moves(&arrow_pad, true);

    let codes = parse_input(input);
    let result = codes.iter().map(|code| {
        let mut current = 'A';
        let mut instructions = String::new();
        for c in code.chars() {
            println!("{} -> {}", current, c);
            let first_robot_moves = keypad_moves
                .get(&current).unwrap()
                .get(&c).unwrap();
            println!("first_robot_moves: {}", first_robot_moves);
            let first_arrow_moves = first_robot_moves.chars()
                .map(|c| arrow_pad_moves.get(&c).unwrap()
                    .get(&current).unwrap()).join("");
            println!("first_arrow_moves: {}", first_arrow_moves);
            let second_arrow_moves = first_arrow_moves.chars()
                .map(|c| arrow_pad_moves.get(&c).unwrap()
                    .get(&c).unwrap()).join("");
            println!("second_arrow_moves: {}", second_arrow_moves);
            let my_moves = second_arrow_moves.chars()
                .map(|c| keypad_moves.get(&c).unwrap()
                    .get(&c).unwrap()).join("");
            println!("my_moves: {}", my_moves);
            current = c;
            instructions = format!("{}{}", instructions, my_moves.as_str());
        }
        instructions.len() as u32 * number(code)
    }).sum();

    println!("{:?}", keypad_moves);
    println!("{:?}", arrow_pad_moves);
    Some(result)
}

fn number(code: &str) -> u32 {
    code.chars().fold(0u32, |acc, c| {
        if c == 'A' {
            return acc;
        }
        acc * 10 + c.to_digit(10).unwrap()
    })
}

fn build_moves(pad: &Vec<Vec<char>>, arrow_pad: bool) -> HashMap<char, HashMap<char, String>> {
    let mut result = HashMap::new();
    for (y, row) in pad.iter().enumerate() {
        for (x, button) in row.iter().enumerate() {
            if *button == 'X' {
                continue;
            }
            result.insert(*button, build_button_moves(&pad, x, y, arrow_pad));
        }
    }
    result
}

fn build_button_moves(pad: &&Vec<Vec<char>>, x: usize, y: usize, arrow_pad: bool) -> HashMap<char, String> {
    let mut dq = VecDeque::new();
    let mut result = HashMap::new();
    dq.push_back((x, y, "".to_string()));
    while !dq.is_empty() {
        let (new_x, new_y, path) = dq.pop_front().unwrap();
        result.insert(pad[new_y][new_x], path.clone());

        for (dx, dy, dir) in vec![(0, 1, 'v'), (1, 0, '>'), (0, -1, '^'), (-1, 0, '<')] {
            let x = new_x.wrapping_add_signed(dx);
            let y = new_y.wrapping_add_signed(dy);
            if x >= pad[0].len() || y >= pad.len() || result.contains_key(&pad[y][x]) || pad[y][x] == 'X' {
                continue;
            }
            let button = pad[y][x];
            let new_path = format!("{}{}", path, dir);
            if arrow_pad {
                result.insert(button, format!("{}A", new_path));
            } else {
                result.insert(button, new_path.clone());
            };

            dq.push_back((x, y, new_path));
        }
    }

    result
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn parse_input(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
