use aoc::{run, Day, Example, Part, Solver};

const INPUT: &str = include_str!("../../inputs/day01/input.txt");
const EXAMPLE_INPUT_ONE: &str = include_str!("../../inputs/day01/part01-example-input.txt");
const EXAMPLE_OUTPUT_ONE: &str = include_str!("../../inputs/day01/part01-example-output.txt");
const EXAMPLE_INPUT_TWO: &str = include_str!("../../inputs/day01/part02-example-input.txt");
const EXAMPLE_OUTPUT_TWO: &str = include_str!("../../inputs/day01/part02-example-output.txt");

struct SolverOne;

impl<'a> Solver<'a, u64> for SolverOne {
    fn name(&self) -> &'a str {
        "calculate distances"
    }

    fn solve(&self, input: &str) -> Option<Result<u64, String>> {
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

        Some(Ok(sum))
    }
}

struct SolverTwo;

impl<'a> Solver<'a, u64> for SolverTwo {
    fn name(&self) -> &'a str {
        "calculate similarity"
    }

    fn solve(&self, input: &str) -> Option<Result<u64, String>> {
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

        Some(Ok(sum))
    }
}

fn main() {
    let example_one = Example::new(
        EXAMPLE_INPUT_ONE,
        EXAMPLE_OUTPUT_ONE.trim().parse().unwrap(),
    );
    let example_two = Example::new(
        EXAMPLE_INPUT_TWO,
        EXAMPLE_OUTPUT_TWO.trim().parse().unwrap(),
    );

    let part_one = Part::new(&SolverOne, example_one);
    let part_two = Part::new(&SolverTwo, example_two);

    let day = Day::new(1, "Historian Hysteria", INPUT, part_one, part_two);

    run(day);
}
