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

#[derive(Debug)]
struct Report {
    pub levels: Vec<u32>,
}

fn parse_input(input: &str) -> Vec<Report> {
    input
        .lines()
        .map(|line| Report {
            levels: line
                .split_whitespace()
                .map(|v| v.parse().unwrap())
                .collect(),
        })
        .collect()
}

// The solution for the first part.
fn solve_one(input: &str) -> String {
    let reports = parse_input(input);

    let mut num_of_safe_reports = 0;

    'reports: for report in reports {
        let is_increasing = report.levels.first().unwrap() < report.levels.get(1).unwrap();

        let mut levels = report.levels.iter();

        let mut lhs = levels.next().unwrap();

        for rhs in levels {
            if (lhs < rhs) != is_increasing {
                continue 'reports;
            }

            if !(1..=3).contains(&lhs.abs_diff(*rhs)) {
                continue 'reports;
            }

            lhs = rhs;
        }

        num_of_safe_reports += 1;
    }

    num_of_safe_reports.to_string()
}

// The solution for the second part.
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
