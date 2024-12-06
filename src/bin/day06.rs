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
    pub const INPUT_TWO: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;
    pub const OUTPUT_TWO: &str = "6";
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Vec2 {
    pub x: usize,
    pub y: usize,
}

impl std::fmt::Display for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{};{}", self.x, self.y)
    }
}

#[derive(Clone, Copy, Debug)]
struct Path {
    pub start: Vec2,
    pub end: Vec2,
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} -> {})", self.start, self.end)
    }
}

impl Path {
    pub fn get_overlap(&self, other: Path) -> Option<Vec2> {
        let self_included_coords = self.included_coords();
        let other_included_coords = other.included_coords();
        dbg!(&self_included_coords);
        dbg!(&other_included_coords);

        self_included_coords
            .iter()
            .find(|coord| other_included_coords.contains(coord))
            .copied()
    }

    pub fn included_coords(&self) -> Vec<Vec2> {
        match self.get_direction() {
            Direction::Up => (self.end.y..=self.start.y)
                .map(|y| Vec2 { x: self.start.x, y })
                .collect(),
            Direction::Down => (self.start.y..=self.end.y)
                .map(|y| Vec2 { x: self.start.x, y })
                .collect(),
            Direction::Right => (self.start.x..=self.end.x)
                .map(|x| Vec2 { x, y: self.start.y })
                .collect(),
            Direction::Left => (self.end.x..=self.start.x)
                .map(|x| Vec2 { x, y: self.start.y })
                .collect(),
        }
    }

    pub fn get_direction(&self) -> Direction {
        if self.start.x == self.end.x {
            if self.start.y < self.end.y {
                return Direction::Down;
            } else {
                return Direction::Up;
            }
        }

        if self.start.y == self.end.y {
            if self.start.x < self.end.x {
                return Direction::Right;
            } else {
                return Direction::Left;
            }
        }

        unreachable!()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tile {
    Obstacle,
    Unvisited,
    Visited,
}

#[derive(Clone, Copy, Debug, PartialEq)]
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

    pub fn as_offset(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

#[derive(Debug)]
struct Map {
    pub width: usize,

    pub tiles: Vec<Tile>,
}

impl Map {
    pub fn count_visited_tiles(&self) -> usize {
        self.tiles
            .iter()
            .filter(|tile| **tile == Tile::Visited)
            .count()
    }

    pub fn visit_tile(&mut self, coords: Vec2) {
        let index = self.coords_to_index(coords);
        self.tiles[index] = Tile::Visited;
    }

    pub fn get(&self, coords: Vec2) -> Option<Tile> {
        self.tiles.get(self.coords_to_index(coords)).copied()
    }

    fn coords_to_index(&self, coords: Vec2) -> usize {
        coords.y * self.width + coords.x
    }
}

fn parse_input(input: &str) -> (Map, Vec2) {
    let width = input.lines().next().unwrap().len();

    let mut tiles = vec![];

    let mut starting_position = Vec2 { x: 0, y: 0 };

    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            match ch {
                '#' => tiles.push(Tile::Obstacle),
                '.' => tiles.push(Tile::Unvisited),
                '^' => {
                    starting_position = Vec2 { x, y };
                    tiles.push(Tile::Visited)
                }
                ch => unreachable!("did not expect to encounter '{}'", ch),
            }
        }
    }

    (Map { width, tiles }, starting_position)
}

// The solution for the first part.
#[cfg(feature = "part-one")]
fn solve_one(input: &str) -> String {
    let mut direction = Direction::Up;
    let (mut map, mut coords) = parse_input(input);

    loop {
        let offset = direction.as_offset();
        let next_coords = (coords.x as isize + offset.0, coords.y as isize + offset.1);
        if next_coords.0 < 0 || next_coords.1 < 0 {
            break;
        }
        let next_coords = Vec2 {
            x: next_coords.0 as usize,
            y: next_coords.1 as usize,
        };

        let next_tile = match map.get(next_coords) {
            Some(next_tile) => next_tile,
            None => break,
        };

        match next_tile {
            Tile::Obstacle => direction = direction.rotate_right(),
            Tile::Unvisited => {
                map.visit_tile(next_coords);
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
fn solve_two(input: &str) -> String {
    let mut direction = Direction::Up;
    let (map, mut coords) = parse_input(input);
    let start_coords = coords;

    let mut num_of_options = 0;

    let mut paths = vec![];
    let mut start = coords;

    loop {
        let offset = direction.as_offset();
        let next_coords = (coords.x as isize + offset.0, coords.y as isize + offset.1);
        if next_coords.0 < 0 || next_coords.1 < 0 {
            break;
        }
        let next_coords = Vec2 {
            x: next_coords.0 as usize,
            y: next_coords.1 as usize,
        };

        let next_tile = match map.get(next_coords) {
            Some(next_tile) => next_tile,
            None => break,
        };

        match next_tile {
            Tile::Obstacle => {
                direction = direction.rotate_right();

                paths.push(Path { start, end: coords });
                start = coords;
            }
            Tile::Unvisited | Tile::Visited => {
                coords = next_coords;
            }
        }
    }

    paths.push(Path { start, end: coords });

    for four_paths in paths.windows(4) {
        let first = four_paths[0];
        let last = four_paths[3];
        println!("checking {} and {} for overlaps", first, last);

        let overlap = first.get_overlap(last);

        if let Some(overlap) = overlap {
            let (offset_x, offset_y) = last.get_direction().as_offset();
            let coords_to_place = Vec2 {
                x: (overlap.x as isize + offset_x) as usize,
                y: (overlap.y as isize + offset_y) as usize,
            };
            if start_coords != coords_to_place {
                println!("overlap at {}", overlap);
                num_of_options += 1;
            }
        }
    }

    num_of_options.to_string()
}

// Creates the main function and a test module
// to run the examples.
aoc_2024::run!(
    INPUT,
    examples::INPUT_ONE => examples::OUTPUT_ONE,
    examples::INPUT_TWO => examples::OUTPUT_TWO
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direction_test() {
        let paths = [
            Path {
                start: Vec2 { x: 3, y: 1 },
                end: Vec2 { x: 3, y: 5 },
            },
            Path {
                start: Vec2 { x: 1, y: 1 },
                end: Vec2 { x: 5, y: 1 },
            },
        ];
        let expected_directions = [Direction::Down, Direction::Right];

        for (path, expected_direction) in std::iter::zip(paths, expected_directions) {
            assert_eq!(path.get_direction(), expected_direction);
        }
    }

    #[test]
    fn included_coords_test() {
        let path = Path {
            start: Vec2 { x: 3, y: 3 },
            end: Vec2 { x: 7, y: 3 },
        };

        assert_eq!(
            path.included_coords(),
            vec![
                Vec2 { x: 3, y: 3 },
                Vec2 { x: 4, y: 3 },
                Vec2 { x: 5, y: 3 },
                Vec2 { x: 6, y: 3 },
                Vec2 { x: 7, y: 3 }
            ]
        )
    }

    #[test]
    fn overlap_test() {
        let path_one = Path {
            start: Vec2 { x: 3, y: 3 },
            end: Vec2 { x: 7, y: 3 },
        };
        let path_two = Path {
            start: Vec2 { x: 5, y: 1 },
            end: Vec2 { x: 5, y: 7 },
        };
        let overlap = path_one.get_overlap(path_two);

        assert_eq!(overlap.unwrap(), Vec2 { x: 5, y: 3 });
    }
}
