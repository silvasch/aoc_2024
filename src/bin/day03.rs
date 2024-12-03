use aoc::{read_example_input, read_example_output, read_input, run, Day, Example, Part, Solver};

const DAY_NUMBER: u32 = 3;

struct Scanner {
    input: Vec<char>,
    current_index: usize,
}

impl Scanner {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            current_index: 0,
        }
    }

    pub fn expect(&self, offset: isize, pattern: &str) -> bool {
        for (i, ch) in pattern.chars().enumerate() {
            if !self.expect_ch(offset + i as isize, ch) {
                return false;
            }
        }

        true
    }

    pub fn expect_ch(&self, offset: isize, ch: char) -> bool {
        self.peek(offset).map(|v| *v == ch).unwrap_or(false)
    }

    pub fn make_number(&mut self) -> Option<u64> {
        let mut raw_number = String::new();

        loop {
            let ch = match self.peek(0) {
                Some(ch) => *ch,
                None => return None,
            };

            if ch.is_ascii_digit() {
                raw_number.push(ch);
            } else {
                break;
            }

            self.advance(1);
        }

        Some(
            raw_number
                .parse()
                .expect("shoud only produce valid numbers"),
        )
    }

    pub fn peek(&self, offset: isize) -> Option<&char> {
        let index = self.current_index as isize + offset;
        let index = if index < 0 {
            return None;
        } else {
            index as usize
        };
        self.input.get(index)
    }

    pub fn advance(&mut self, offset: usize) {
        self.current_index += offset;
    }
}

struct SolverOne;

impl<'a> Solver<'a, u64> for SolverOne {
    fn name(&self) -> &'a str {
        "multiplications"
    }

    fn solve(&self, input: &str) -> Option<Result<u64, String>> {
        let mut sum = 0;

        let mut scanner = Scanner::new(input);

        while let Some(_) = scanner.peek(0) {
            if scanner.expect(0, "mul(") {
                scanner.advance(4);
                let lhs = match scanner.make_number() {
                    Some(lhs) => lhs,
                    None => {
                        continue;
                    }
                };
                if !scanner.expect_ch(0, ',') {
                    continue;
                }
                scanner.advance(1);
                let rhs = match scanner.make_number() {
                    Some(rhs) => rhs,
                    None => {
                        continue;
                    }
                };
                if !scanner.expect_ch(0, ')') {
                    continue;
                }
                scanner.advance(1);
                sum += lhs * rhs;
            } else {
                scanner.advance(1);
            }
        }

        Some(Ok(sum))
    }
}

struct SolverTwo;

impl<'a> Solver<'a, u64> for SolverTwo {
    fn name(&self) -> &'a str {
        "multiplications"
    }

    fn solve(&self, input: &str) -> Option<Result<u64, String>> {
        let mut sum = 0;

        let mut scanner = Scanner::new(input);

        let mut do_count = true;

        while let Some(_) = scanner.peek(0) {
            if scanner.expect(0, "mul(") {
                scanner.advance(4);
                let lhs = match scanner.make_number() {
                    Some(lhs) => lhs,
                    None => {
                        continue;
                    }
                };
                if !scanner.expect_ch(0, ',') {
                    continue;
                }
                scanner.advance(1);
                let rhs = match scanner.make_number() {
                    Some(rhs) => rhs,
                    None => {
                        continue;
                    }
                };
                if !scanner.expect_ch(0, ')') {
                    continue;
                }
                scanner.advance(1);
                if do_count {
                    sum += lhs * rhs;
                }
            } else if scanner.expect(0, "do()") {
                scanner.advance(4);
                do_count = true;
            } else if scanner.expect(0, "don't()") {
                scanner.advance(7);
                do_count = false;
            } else {
                scanner.advance(1);
            }
        }

        Some(Ok(sum))
    }
}

fn main() {
    let input = read_input(DAY_NUMBER).unwrap_or_default();
    let example_input_one = read_example_input(DAY_NUMBER, 1).unwrap_or_default();
    let example_output_one = read_example_output::<u64>(DAY_NUMBER, 1).unwrap_or_default();
    let example_input_two = read_example_input(DAY_NUMBER, 2).unwrap_or_default();
    let example_output_two = read_example_output::<u64>(DAY_NUMBER, 2).unwrap_or_default();

    let example_one = Example::new(&example_input_one, example_output_one);
    let example_two = Example::new(&example_input_two, example_output_two);

    let part_one = Part::new(&SolverOne, example_one);
    let part_two = Part::new(&SolverTwo, example_two);

    let day = Day::new(1, "Mull It Over", &input, part_one, part_two);

    run(day);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expect_ch_test() {
        let mut scanner = Scanner::new("abc");
        assert!(scanner.expect_ch(0, 'a'));
        assert!(!scanner.expect_ch(1, 'a'));
        scanner.advance(2);
        assert!(scanner.expect_ch(0, 'c'));
    }

    #[test]
    fn expect_test() {
        let scanner = Scanner::new("abc");
        assert!(scanner.expect(0, "abc"));
    }

    #[test]
    fn make_number_test() {
        let mut scanner = Scanner::new("123b");
        assert_eq!(scanner.make_number(), Some(123));
        assert!(scanner.expect_ch(0, 'b'));
    }
}
