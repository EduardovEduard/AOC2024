use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let regex: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let res = regex.captures_iter(input).fold(0i32, |acc, cap| {
        let (_, [a, b]) = cap.extract();
        acc + a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap()
    });
    Some(res as u32)
}

const DIGITS: &str = "1234567890";

struct Parser {
    chars: Vec<char>,
    i: Option<usize>,
    remaining: usize,
    do_state: Option<Do>
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Do {
    DO,
    DONT
}

impl Parser {
    fn new(input: &str) -> Self {
        Self {
            chars: input.chars().collect(),
            i: Some(0),
            remaining: input.chars().count(),
            do_state: None
        }
    }

    fn parse_next_mul(&mut self) -> Option<(u32, u32)> {
        while self.current() != Some('m') && self.current() != Some('d') {
            self.advance()?;
        }
        match self.current() {
            Some('d') => {
                self.do_state = self.parse_do();
                self.parse_next_mul()
            },
            Some('m') => {
                match self.parse_mul() {
                    Some((a, b)) if self.do_state != Some(Do::DONT) => {
                        Some((a, b))
                    },
                    Some(_) if self.do_state == Some(Do::DONT) => {
                        self.parse_next_mul()
                    }
                    _ => self.parse_next_mul()
                }
            },
            _ => {
                self.advance();
                self.parse_next_mul()
            }
        }
    }

    fn parse_do(&mut self) -> Option<Do> {
        if self.peek("don't()") {
            self.advance_by(7)?;
            Some(Do::DONT)
        } else if self.peek("do()") {
            self.advance_by(4)?;
            Some(Do::DO)
        } else {
            self.advance();
            self.do_state
        }
    }

    fn parse_mul(&mut self) -> Option<(u32, u32)> {
        if self.peek("mul(") {
            self.advance_by(4)?;
            let a= self.parse_number()?;
            self.expect_char(',');
            let b = self.parse_number()?;
            self.expect_char(')')?;
            Some((a, b))
        } else {
            self.advance();
            None
        }
    }

    fn parse_number(&mut self) -> Option<u32> {
        let mut number = 0;
        let mut c = self.current();
        while c.is_some() && DIGITS.contains(c.unwrap()) {
            number = number * 10 + c.unwrap().to_digit(10).unwrap();
            c = self.advance();
        }
        if number > 999 {
            return None
        }
        Some(number)
    }

    fn peek(&self, text: &str) -> bool {
        if let Some(i) = self.i {
            self.remaining >= text.len() && self.chars[i..i + text.len()] == text.chars().collect::<Vec<char>>()
        } else {
            false
        }
    }

    fn expect_char(&mut self, char: char) -> Option<char> {
        let c = self.current();
        if c.is_some() && c.unwrap() == char {
            self.advance();
            Some(c.unwrap())
        } else {
            None
        }
    }

    fn current(&self) -> Option<char> {
        self.i.and_then(|i| self.chars.get(i).copied())
    }

    fn advance(&mut self) -> Option<char> {
        let i = self.i?;
        if i == self.chars.len() {
            return None
        }
        self.i = Some(i + 1);
        self.remaining -= 1;
        self.current()
    }

    fn advance_by(&mut self, n: usize) -> Option<()> {
        for _ in 0..n {
            self.advance()?;
        }
        Some(())
    }
}

impl Iterator for Parser {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        self.parse_next_mul()
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let parser = Parser::new(input);
    Some(parser.fold(0, |acc, (a, b)| acc + a * b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
