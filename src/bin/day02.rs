// Read the input from /inputs/day{}.txt
const INPUT: &str = aoc_2024::read_input!("02");

// All these variables are only used in tests,
// which is why rust-analyzer thinks they are unused.
#[allow(unused)]
mod examples {
    pub const INPUT_ONE: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;
    pub const OUTPUT_ONE: &str = "2";
    pub const INPUT_TWO: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;
    pub const OUTPUT_TWO: &str = "4";
}

// The solution for the first part.
#[cfg(feature = "part-one")]
fn solve_one(_input: &str) -> String {
    todo!()
}

// The solution of the second part.
#[cfg(feature = "part-two")]
fn solve_two(_input: &str) -> String {
    todo!()
}

// Creates the main function and a test module
// to run the examples.
aoc_2024::run!(
    INPUT,
    examples::INPUT_ONE => examples::OUTPUT_ONE,
    examples::INPUT_TWO => examples::OUTPUT_TWO
);
