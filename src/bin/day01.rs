// Read the input from /inputs/day{}.txt
const INPUT: &str = aoc_2024::read_input!("01");

// All these variables are only used in tests,
// which is why rust-analyzer thinks they are unused.
#[allow(unused)]
mod examples {
    pub const INPUT_ONE: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;
    pub const OUTPUT_ONE: &str = "11";
    pub const INPUT_TWO: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;
    pub const OUTPUT_TWO: &str = "31";
}

// The solution for the first part.
#[cfg(feature = "part-one")]
fn solve_one(input: &str) -> String {
    let mut sum = 0;

    let mut left_column: Vec<u64> = vec![];
    let mut right_column: Vec<u64> = vec![];

    for line in input.lines() {
        let mut split_line = line.split_whitespace();
        left_column.push(split_line.next().unwrap().parse().unwrap());
        right_column.push(split_line.next().unwrap().parse().unwrap());
    }

    left_column.sort_unstable();
    right_column.sort_unstable();

    for i in 0..left_column.len() {
        let left_number = left_column.get(i).unwrap();
        let right_number = right_column.get(i).unwrap();
        sum += left_number.abs_diff(*right_number);
    }

    sum.to_string()
}

// The solution for the second part.
#[cfg(feature = "part-two")]
fn solve_two(input: &str) -> String {
    let mut sum = 0;

    let mut left_column: Vec<u64> = vec![];
    let mut right_column: Vec<u64> = vec![];

    for line in input.lines() {
        let mut split_line = line.split_whitespace();
        left_column.push(split_line.next().unwrap().parse().unwrap());
        right_column.push(split_line.next().unwrap().parse().unwrap());
    }

    for left_number in left_column {
        let mut count = 0;

        for right_number in &right_column {
            if left_number == *right_number {
                count += 1;
            }
        }

        sum += count * left_number;
    }

    sum.to_string()
}

// Creates the main function and a test module
// to run the examples.
aoc_2024::run!(
    INPUT,
    examples::INPUT_ONE => examples::OUTPUT_ONE,
    examples::INPUT_TWO => examples::OUTPUT_TWO
);
