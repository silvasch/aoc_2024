// Read the input from /inputs/day-{}.txt
const INPUT: &str = aoc_2024::read_input!("00");

// All these variables are only used in tests,
// which is why rust-analyzer thinks they are unused.
#[allow(unused)]
mod examples {
    pub const INPUT_ONE: &str = "";
    pub const OUTPUT_ONE: &str = "";
    pub const INPUT_TWO: &str = "";
    pub const OUTPUT_TWO: &str = "";
}

// The solution for the first part.
#[cfg(feature = "part-one")]
fn solve_one(
    #[expect(
        unused,
        reason = "now that your solution uses `input`, this `expect` attribute can be removed."
    )]
    input: &str,
) -> String {
    todo!()
}

// The solution of the second part.
#[cfg(feature = "part-two")]
fn solve_two(
    #[expect(
        unused,
        reason = "now that your solution uses `input`, this `expect` attribute can be removed."
    )]
    input: &str,
) -> String {
    todo!()
}

// Creates the main function and a test module
// to run the examples.
aoc_2024::run!(
    INPUT,
    examples::INPUT_ONE => examples::OUTPUT_ONE,
    examples::INPUT_TWO => examples::OUTPUT_TWO
);
