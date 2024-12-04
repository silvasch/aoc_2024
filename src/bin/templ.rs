// Read the input from /inputs/day-{}.txt
const INPUT: &str = aoc_2024::read_input!("00");

// All these variables are only used in tests,
// which is why rust-analyzer thinks they are unused.
#[allow(unused)]
mod examples {
    pub const INPUT_ONE: &str = "2 3";
    pub const OUTPUT_ONE: &str = "5";
    pub const INPUT_TWO: &str = "2 3";
    pub const OUTPUT_TWO: &str = "6";
}

// The solution for the first part.
#[cfg(feature = "part-one")]
fn solve_one(input: &str) -> String {
    input
        .split_whitespace()
        .map(|v| v.parse::<i32>().unwrap())
        .sum::<i32>()
        .to_string()
}

// The solution of the second part.
#[cfg(feature = "part-two")]
fn solve_two(input: &str) -> String {
    input
        .split_whitespace()
        .map(|v| v.parse::<i32>().unwrap())
        .product::<i32>()
        .to_string()
}

// Creates the main function and a test module
// to run the examples.
aoc_2024::run!(
    INPUT,
    examples::INPUT_ONE => examples::OUTPUT_ONE,
    examples::INPUT_TWO => examples::OUTPUT_TWO
);
