use aoc::{read_example_input, read_example_output, read_input, run, Day, Example, Part, Solver};

const DAY_NUMBER: u32 = 2;

fn is_valid_pair(lhs: u32, rhs: u32, should_increase: bool) -> bool {
    if lhs == rhs {
        return false;
    }

    let diff = lhs.abs_diff(rhs);
    if diff > 3 || diff < 1 {
        return false;
    }

    if (rhs > lhs) != should_increase {
        return false;
    }

    true
}

struct SolverOne;

impl<'a> Solver<'a, u64> for SolverOne {
    fn name(&self) -> &'a str {
        "safe reports"
    }

    fn solve(&self, input: &str) -> Option<Result<u64, String>> {
        let mut number_of_safe_repots = 0;

        'reports: for report in input.lines() {
            let mut levels = report.split_whitespace().map(|v| v.parse::<u32>().unwrap());

            let first_level = levels.next().unwrap();
            let mut last_level = levels.next().unwrap();

            let should_increase = last_level > first_level;

            if is_valid_pair(first_level, last_level, should_increase) {
                continue;
            }

            for level in levels {
                if is_valid_pair(last_level, level, should_increase) {
                    continue 'reports;
                }

                last_level = level;
            }

            number_of_safe_repots += 1;
        }

        Some(Ok(number_of_safe_repots))
    }
}

struct SolverTwo;

impl<'a> Solver<'a, u64> for SolverTwo {
    fn name(&self) -> &'a str {
        "tolerated safe reports"
    }

    fn solve(&self, input: &str) -> Option<Result<u64, String>> {
        let mut num_of_safe_reports = 0;

        Some(Ok(num_of_safe_reports))
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

    let day = Day::new(1, "Red-Nosed Reports", &input, part_one, part_two);

    run(day);
}
