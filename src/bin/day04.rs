use aoc_2024b::read_input;

const INPUT: &str = read_input!(4);

fn get_char_from_coords(input: &str, coords: (usize, usize)) -> Option<char> {
    let line = input.lines().nth(coords.1)?;
    let ch = line.chars().nth(coords.0)?;
    Some(ch)
}

#[cfg(feature = "part-one")]
#[tracing::instrument(skip(input))]
fn solve_one(input: &str) -> Result<String, anyhow::Error> {
    let mut word_count = 0;

    let mut lines = vec![];

    // horizontal lines
    tracing::debug!("now generating horizontal lines");
    for line in input.lines() {
        let new_line = line.to_string();
        tracing::debug!("new line: {}", new_line);
        lines.push(line.to_string());
    }

    // vertical lines
    tracing::debug!("now generating vertical lines");
    let line_length = input.lines().next().unwrap().len();
    for index in 0..line_length {
        let mut new_line = String::new();
        for line in input.lines() {
            new_line.push(line.chars().nth(index).unwrap());
        }
        tracing::debug!("new line: {}", new_line);
        lines.push(new_line);
    }

    tracing::debug!("now generating diagonal lines from the top left");
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

        tracing::debug!("new line: {}", new_line);
        lines.push(new_line);
    }

    tracing::debug!("now generating diagonal lines from the top right");
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

        tracing::debug!("{:?}", (start_x, start_y));

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

        tracing::debug!("new line: {}", new_line);
        lines.push(new_line);
    }

    for line in &lines {
        word_count += line.matches("XMAS").count();
        word_count += line.matches("SAMX").count();
    }

    Ok(word_count.to_string())
}

#[cfg(feature = "part-two")]
#[tracing::instrument(skip(input))]
fn solve_two(input: &str) -> Result<String, anyhow::Error> {
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

    Ok(count.to_string())
}

fn main() {
    tracing_subscriber::fmt::init();

    #[cfg(feature = "part-one")]
    println!("{}", solve_one(INPUT).unwrap());

    #[cfg(feature = "part-two")]
    println!("{}", solve_two(INPUT).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "part-one")]
    #[tracing_test::traced_test]
    #[test]
    fn part_one_test() {
        let input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;
        let expected_output = "18".to_string();

        let output = solve_one(&input).unwrap();

        assert_eq!(output, expected_output);
    }

    #[cfg(feature = "part-two")]
    #[tracing_test::traced_test]
    #[test]
    fn part_two_test() {
        let input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;
        let expected_output = "9".to_string();

        let output = solve_two(&input).unwrap();

        assert_eq!(output, expected_output);
    }
}
