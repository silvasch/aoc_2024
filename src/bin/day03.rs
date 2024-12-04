// Read the input from /inputs/day-{}.txt
const INPUT: &str = aoc_2024b::read_input!("03");

// All these variables are only used in tests,
// which is why rust-analyzer thinks they are unused.
#[allow(unused)]
mod examples {
    pub const INPUT_ONE: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    pub const OUTPUT_ONE: &str = "161";
    pub const INPUT_TWO: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    pub const OUTPUT_TWO: &str = "48";
}

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

// The solution for the first part.
#[cfg(feature = "part-one")]
fn solve_one(input: &str) -> String {
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

    sum.to_string()
}

// The solution of the second part.
#[cfg(feature = "part-two")]
fn solve_two(input: &str) -> String {
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

    sum.to_string()
}

// Creates the main function and a test module
// to run the examples.
aoc_2024b::run!(
    INPUT,
    examples::INPUT_ONE => examples::OUTPUT_ONE,
    examples::INPUT_TWO => examples::OUTPUT_TWO
);
