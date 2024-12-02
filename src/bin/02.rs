advent_of_code::solution!(2);

struct Report {
    pub levels: Vec<i32>,
}

trait SafeDiff {
    fn is_safe(&self) -> bool;
}

impl SafeDiff for i32 {
    fn is_safe(&self) -> bool {
        self >= &1 && self <= &3
    }
}

impl Report {
    fn is_safe(&self) -> bool {
        let sign = (self.levels[0] - self.levels[1]).signum();
        let diff = (self.levels[0] - self.levels[1]).abs();
        if !diff.is_safe() {
            return false;
        }

        for i in 1..self.levels.len() - 1 {
            let new_sign = (self.levels[i] - self.levels[i + 1]).signum();
            let new_diff = (self.levels[i] - self.levels[i + 1]).abs();
            if sign != new_sign || !new_diff.is_safe() {
                return false;
            }
        }
        true
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let reports = parse_input(input);
    Some(reports.iter().filter(|&report| report.is_safe()).count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let reports = parse_input(input);
    let mut count = 0;

    for report in &reports {
        if report.is_safe() {
            count += 1
        } else {
            for i in 0..report.levels.len() {
                let new_levels = [&report.levels[0..i], &report.levels[i + 1..]].concat();
                let new_report = Report {
                    levels: new_levels
                };
                if new_report.is_safe() {
                    count += 1;
                    break
                }
            }
        }
    }

    Some(count)
}

fn parse_input(input: &str) -> Vec<Report> {
    input.lines()
        .map(|line| Report {
            levels: line.split_ascii_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
            })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
