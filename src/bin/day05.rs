use std::cmp::Ordering;

// Read the input from /inputs/day-{}.txt
const INPUT: &str = aoc_2024::read_input!("05");

// All these variables are only used in tests,
// which is why rust-analyzer thinks they are unused.
#[allow(unused)]
mod examples {
    pub const INPUT_ONE: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;
    pub const OUTPUT_ONE: &str = "143";
    pub const INPUT_TWO: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;
    pub const OUTPUT_TWO: &str = "123";
}

#[derive(Debug)]
struct Rule {
    pub before: u32,
    pub after: u32,
}

#[derive(Debug)]
struct Update {
    pub pages: Vec<u32>,
}

impl Update {
    pub fn follows_rules(&self, rules: &[Rule]) -> bool {
        for rule in rules {
            let before_index = match self.pages.iter().position(|page| *page == rule.before) {
                Some(before_index) => before_index,
                None => continue,
            };
            let after_index = match self.pages.iter().position(|page| *page == rule.after) {
                Some(after_index) => after_index,
                None => continue,
            };

            if after_index < before_index {
                return false;
            }
        }

        true
    }

    pub fn order_pages(&mut self, rules: &[Rule]) {
        // TODO(challenge): implement own sorting algorithm
        self.pages.sort_by(|a, b| {
            for rule in rules {
                if *a == rule.before && *b == rule.after {
                    return Ordering::Less;
                } else if *a == rule.after && *b == rule.before {
                    return Ordering::Greater;
                }
            }
            Ordering::Less
        })
    }

    pub fn middle_page(&self) -> u32 {
        *self.pages.get(self.pages.len().div_ceil(2) - 1).unwrap()
    }
}

fn parse_input(input: &str) -> (Vec<Rule>, Vec<Update>) {
    let mut split_input = input.split("\n\n");
    let raw_rules = split_input.next().unwrap();
    let raw_updates = split_input.next().unwrap();

    let rules: Vec<_> = raw_rules
        .lines()
        .map(|line| {
            let mut split_line = line.split("|");
            let before = split_line.next().unwrap().parse().unwrap();
            let after = split_line.next().unwrap().parse().unwrap();
            Rule { before, after }
        })
        .collect();

    let updates: Vec<_> = raw_updates
        .lines()
        .map(|line| {
            let pages = line
                .split(',')
                .map(|raw_page| raw_page.parse().unwrap())
                .collect();
            Update { pages }
        })
        .collect();

    (rules, updates)
}

// The solution for the first part.
#[cfg(feature = "part-one")]
fn solve_one(input: &str) -> String {
    let (rules, updates) = parse_input(input);

    let mut sum = 0;

    for update in updates {
        if update.follows_rules(&rules) {
            sum += update.middle_page();
        }
    }

    sum.to_string()
}

// The solution of the second part.
#[cfg(feature = "part-two")]
fn solve_two(input: &str) -> String {
    let (rules, mut updates) = parse_input(input);

    let mut sum = 0;

    for update in updates.iter_mut() {
        if update.follows_rules(&rules) {
            continue;
        }
        update.order_pages(&rules);
        sum += update.middle_page();
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