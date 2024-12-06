// Read the input from /inputs/day-{}.txt
const INPUT: &str = aoc_2024::read_input!("06");

// All these variables are only used in tests,
// which is why rust-analyzer thinks they are unused.
#[allow(unused)]
mod examples {
    pub const INPUT_ONE: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;
    pub const OUTPUT_ONE: &str = "41";
    pub const INPUT_TWO: &str = "";
    pub const OUTPUT_TWO: &str = "";
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tile {
    Obstacle,
    Unvisited,
    Visited,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn rotate_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Debug)]
struct Map {
    pub width: usize,
    pub height: usize,

    pub tiles: Vec<Tile>,
}

impl Map {
    pub fn count_visited_tiles(&self) -> usize {
        self.tiles
            .iter()
            .filter(|tile| **tile == Tile::Visited)
            .count()
    }

    pub fn visit_tile(&mut self, x: usize, y: usize) {
        let index = self.coords_to_index(x, y);
        self.tiles[index] = Tile::Visited;
    }

    pub fn get(&self, x: usize, y: usize) -> Option<Tile> {
        self.tiles.get(self.coords_to_index(x, y)).copied()
    }

    fn coords_to_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }
}

fn parse_input(input: &str) -> (Map, (usize, usize)) {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    let mut tiles = vec![];

    let mut starting_position = (0, 0);

    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            match ch {
                '#' => tiles.push(Tile::Obstacle),
                '.' => tiles.push(Tile::Unvisited),
                '^' => {
                    starting_position = (x, y);
                    tiles.push(Tile::Visited)
                }
                ch => unreachable!("did not expect to encounter '{}'", ch),
            }
        }
    }

    (
        Map {
            width,
            height,
            tiles,
        },
        starting_position,
    )
}

// The solution for the first part.
#[cfg(feature = "part-one")]
fn solve_one(input: &str) -> String {
    let mut direction = Direction::Up;
    let (mut map, mut coords) = parse_input(input);

    loop {
        let offset = match direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        let next_coords = (coords.0 as isize + offset.0, coords.1 as isize + offset.1);
        if next_coords.0 < 0 || next_coords.1 < 0 {
            break;
        }
        let next_coords = (next_coords.0 as usize, next_coords.1 as usize);

        let next_tile = match map.get(next_coords.0, next_coords.1) {
            Some(next_tile) => next_tile,
            None => break,
        };

        match next_tile {
            Tile::Obstacle => direction = direction.rotate_right(),
            Tile::Unvisited => {
                map.visit_tile(next_coords.0, next_coords.1);
                coords = next_coords;
            }
            Tile::Visited => {
                coords = next_coords;
            }
        }
    }

    map.count_visited_tiles().to_string()
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
