#[macro_export]
macro_rules! read_input {
    ($day:literal) => {
        include_str!(concat!(
            location_macros::workspace_dir!(),
            "/inputs/day",
            $day,
            ".txt"
        ))
    };
}

#[macro_export]
macro_rules! run {
    (
        $input:expr,
        $example_input_one:expr => $example_output_one:expr,
        $example_input_two:expr => $example_output_two:expr
    ) => {
        fn main() {
            #[cfg(all(feature = "part-one", feature = "part-two"))]
            {
                run_part_one();
                eprintln!();
                run_part_two();
            }

            #[cfg(all(feature = "part-one", not(feature = "part-two")))]
            run_part_one();

            #[cfg(all(not(feature = "part-one"), feature = "part-two"))]
            run_part_two();
        }

        #[cfg(feature = "part-one")]
        fn run_part_one() {
            eprintln!("part one:");
            println!("{}", solve_one($input));
        }

        #[cfg(feature = "part-two")]
        fn run_part_two() {
            eprintln!("part two:");
            println!("{}", solve_two($input));
        }

        #[cfg(test)]
        mod examples_tests {
            use super::*;

            #[cfg(feature = "part-one")]
            #[test]
            fn part_one_test() {
                assert_eq!(solve_one($example_input_one), $example_output_one);
            }

            #[cfg(feature = "part-two")]
            #[test]
            fn part_two_test() {
                assert_eq!(solve_two($example_input_two), $example_output_two);
            }
        }
    };
}
