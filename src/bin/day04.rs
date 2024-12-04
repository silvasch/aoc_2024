use aoc_2024b::{read_input, run};

const INPUT: &str = read_input!("04");

// All these variables are only used in tests,
// which is why rust-analyzer thinks they are unused.
#[allow(unused)]
const EXAMPLE_INPUT_ONE: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

#[allow(unused)]
const EXAMPLE_OUTPUT_ONE: &str = "18";

#[allow(unused)]
const EXAMPLE_INPUT_TWO: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

#[allow(unused)]
const EXAMPLE_OUTPUT_TWO: &str = "9";

fn get_char_from_coords(input: &str, coords: (usize, usize)) -> Option<char> {
    let line = input.lines().nth(coords.1)?;
    let ch = line.chars().nth(coords.0)?;
    Some(ch)
}

#[cfg(feature = "part-one")]
fn solve_one(input: &str) -> String {
    let mut word_count = 0;

    let mut lines = vec![];

    // horizontal lines
    for line in input.lines() {
        lines.push(line.to_string());
    }

    // vertical lines
    let line_length = input.lines().next().unwrap().len();
    for index in 0..line_length {
        let mut new_line = String::new();
        for line in input.lines() {
            new_line.push(line.chars().nth(index).unwrap());
        }
        lines.push(new_line);
    }

    let mut starts = vec![];

    for (i, _) in input.lines().enumerate() {
        starts.push((0, i));
    }

    for i in 1..input.lines().nth(0).unwrap().chars().count() {
        starts.push((i, 0));
    }

    for (start_x, start_y) in starts {
        let mut new_line = String::new();
        let mut offset = 0;

        loop {
            let line = match input.lines().nth(start_y + offset) {
                Some(line) => line,
                None => break,
            };
            let ch = match line.chars().nth(start_x + offset) {
                Some(line) => line,
                None => break,
            };
            new_line.push(ch);
            offset += 1;
        }

        lines.push(new_line);
    }

    let mut starts = vec![];

    for i in 0..input.lines().count() {
        starts.push((input.lines().next().unwrap().len() - 1, i));
    }

    for i in 0..input.lines().next().unwrap().chars().count() - 1 {
        starts.push((i, 0));
    }

    for (start_x, start_y) in starts {
        let mut new_line = String::new();
        let mut offset = 0;

        loop {
            if start_x < offset {
                break;
            }

            let line = match input.lines().nth(start_y + offset) {
                Some(line) => line,
                None => break,
            };
            let ch = match line.chars().nth(start_x - offset) {
                Some(ch) => ch,
                None => break,
            };
            new_line.push(ch);
            offset += 1;
        }

        lines.push(new_line);
    }

    for line in &lines {
        word_count += line.matches("XMAS").count();
        word_count += line.matches("SAMX").count();
    }

    word_count.to_string()
}

#[cfg(feature = "part-two")]
fn solve_two(input: &str) -> String {
    let mut count = 0;

    let line_length = input.lines().next().unwrap().len();

    for y in 0..input.lines().count() {
        for x in 0..line_length {
            let top_left_ch = get_char_from_coords(input, (x, y));
            let center_ch = get_char_from_coords(input, (x + 1, y + 1));
            let top_right_ch = get_char_from_coords(input, (x + 2, y));
            let bottom_left_ch = get_char_from_coords(input, (x, y + 2));
            let bottom_right_ch = get_char_from_coords(input, (x + 2, y + 2));

            if center_ch != Some('A') {
                continue;
            }

            if ((top_left_ch == Some('M') && bottom_right_ch == Some('S'))
                || (top_left_ch == Some('S') && bottom_right_ch == Some('M')))
                && ((top_right_ch == Some('M') && bottom_left_ch == Some('S'))
                    || (top_right_ch == Some('S') && bottom_left_ch == Some('M')))
            {
                count += 1;
            }
        }
    }

    count.to_string()
}

run!(
    INPUT,
    EXAMPLE_INPUT_ONE => EXAMPLE_OUTPUT_ONE,
    EXAMPLE_INPUT_TWO => EXAMPLE_OUTPUT_TWO
);
