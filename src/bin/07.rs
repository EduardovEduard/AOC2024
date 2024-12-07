advent_of_code::solution!(7);


#[derive(Debug, Clone, Copy)]
enum Op {
    ADD,
    MUL,
    CONCAT
}

impl Op {
    fn apply(&self, a: u64, b: u64) -> u64 {
        match self {
            Op::ADD => a + b,
            Op::MUL => a * b,
            Op::CONCAT => a * 10_u64.pow(b.ilog10() + 1) + b
        }
    }
}

fn permute(ops: &mut Vec<Op>, n: usize, input: &Vec<u64>, result: u64, with_concat: bool) -> Option<u64> {
    let current = input[0];
    let actual = input[1..].iter().zip(ops.iter()).fold(current, |acc, (x, op)| {
        op.apply(acc, *x)
    });

    if actual == result {
        println!("{:?} = {}", ops, result);
        return Some(result);
    }

    if n == ops.len() {
        return None;
    }

    ops[n] = Op::MUL;
    if let res @ Some(_) = permute(ops, n + 1, input, result, with_concat) {
        return res;
    }

    if with_concat {
        ops[n] = Op::CONCAT;
        if let res @ Some(_) = permute(ops, n + 1, input, result, with_concat) {
            return res;
        }
    }

    ops[n] = Op::ADD;
    permute(ops, n + 1, input, result, with_concat)
}

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse_input(input);
    println!("{:?}", input);
    let res = input.iter().filter(|(ans, rest)| {
        let mut ops = vec![Op::ADD; rest.len() - 1];
        permute(&mut ops, 0, rest, *ans, false).is_some()
    }).map(|(ans, _)| ans).sum();
    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = parse_input(input);
    println!("{:?}", input);
    let res = input.iter().filter(|(ans, rest)| {
        let mut ops = vec![Op::ADD; rest.len() - 1];
        permute(&mut ops, 0, rest, *ans, true).is_some()
    }).map(|(ans, _)| ans).sum();
    Some(res)
}

fn parse_input(input: &str) -> Vec<(u64, Vec<u64>)> {
    input.lines().map(|line| {
        let (ans, rest) = line.split_once(":").unwrap();
        let rest: Vec<u64> = rest.trim().split(" ")
            .map(|x| x.trim().parse().unwrap())
            .collect();
        (ans.parse().unwrap(), rest)
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
