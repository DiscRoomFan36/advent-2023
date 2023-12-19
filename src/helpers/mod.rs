#[allow(unused)]
pub mod constructor {
    use std::str::FromStr;

    pub use grid::Grid;

    pub trait FromChar {
        fn from_char(c: char) -> Self;
    }

    pub fn file_to_vec_of_vec<T: FromChar>(file: &str) -> Vec<Vec<T>> {
        file.lines()
            .map(|line| line.chars().map(|c| T::from_char(c)).collect())
            .collect()
    }

    pub fn file_to_grid<T: FromChar + Clone>(file: &str) -> Grid<T> {
        let grid = file_to_vec_of_vec(file);
        Grid::from_vec(grid.concat(), grid[0].len())
    }

    use once_cell::sync::Lazy;
    use regex::Regex;
    fn line_to_digits<T: FromStr + Default>(line: &str) -> Vec<T> {
        const REGEX: &str = r"(-?\d+)";
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(REGEX).unwrap());
        RE.find_iter(line)
            .map(|m| m.as_str().trim_end().parse().unwrap_or_default())
            .collect()
    }
}

#[allow(unused)]
pub mod grid_stuff {
    use super::enums_and_types::Position;
    use grid::Grid;

    pub fn flood_fill(flooder_grid: &mut Grid<bool>, pos: Position) {
        let mut flood_stack = vec![pos];

        while let Some((y, x)) = flood_stack.pop() {
            flooder_grid[(y, x)] = true;

            if x > 0 && (flooder_grid[(y, x - 1)] == false) {
                flood_stack.push((y, x - 1));
            }
            if x < flooder_grid.cols() - 1 && (flooder_grid[(y, x + 1)] == false) {
                flood_stack.push((y, x + 1));
            }

            if y > 0 && (flooder_grid[(y - 1, x)] == false) {
                flood_stack.push((y - 1, x));
            }
            if y < flooder_grid.rows() - 1 && (flooder_grid[(y + 1, x)] == false) {
                flood_stack.push((y + 1, x));
            }
        }
    }
}

#[allow(unused)]
pub mod enums_and_types {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum CompassDirection {
        North,
        South,
        East,
        West,
    }

    pub type Position = (usize, usize);
}

#[allow(unused)]
pub mod positions_and_directions {
    use super::enums_and_types::{Direction, Position};
    use grid::Grid;

    pub fn next_position((j, i): Position, dir: Direction) -> Position {
        match dir {
            Direction::Up => (j - 1, i),
            Direction::Down => (j + 1, i),
            Direction::Left => (j, i - 1),
            Direction::Right => (j, i + 1),
        }
    }

    pub fn next_position_counted<T>((j, i): (T, T), dir: Direction, count: T) -> (T, T)
    where
        T: std::ops::Add<Output = T> + std::ops::Sub<Output = T>,
    {
        match dir {
            Direction::Up => (j - count, i),
            Direction::Down => (j + count, i),
            Direction::Left => (j, i - count),
            Direction::Right => (j, i + count),
        }
    }

    pub fn next_position_bound<T>(
        grid: &Grid<T>,
        (j, i): Position,
        dir: Direction,
    ) -> Option<Position> {
        match dir {
            Direction::Up => {
                if j > 0 {
                    Some((j - 1, i))
                } else {
                    None
                }
            }
            Direction::Down => {
                if j < grid.rows() - 1 {
                    Some((j + 1, i))
                } else {
                    None
                }
            }
            Direction::Left => {
                if i > 0 {
                    Some((j, i - 1))
                } else {
                    None
                }
            }
            Direction::Right => {
                if i < grid.cols() - 1 {
                    Some((j, i + 1))
                } else {
                    None
                }
            }
        }
    }
}

#[allow(unused)]
pub mod print_helpers {
    use grid::Grid;

    pub trait ToChar {
        fn to_char(&self) -> char;
    }

    impl ToChar for bool {
        fn to_char(&self) -> char {
            match self {
                true => '#',
                false => '.',
            }
        }
    }

    pub fn print_grid<T: ToChar>(grid: &Grid<T>) {
        for row in grid.iter_rows() {
            for item in row {
                print!("{}", item.to_char())
            }
            println!()
        }
        println!()
    }
}

#[allow(unused)]
pub mod color {

    pub fn to_hex(char: char) -> u8 {
        match char {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'a' => 10,
            'b' => 11,
            'c' => 12,
            'd' => 13,
            'e' => 14,
            'f' => 15,
            _ => panic!("not a color"),
        }
    }

    pub fn hex_to_bin(hex: &str) -> usize {
        hex.chars().fold(0, |z, u| z * 16 + to_hex(u) as usize)
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Color {
        pub r: u8,
        pub g: u8,
        pub b: u8,
    }
    impl Color {
        pub fn from_string(str: &str) -> Self {
            let mut chars = str.chars();
            let r1 = to_hex(chars.next().unwrap());
            let r2 = to_hex(chars.next().unwrap());

            let g1 = to_hex(chars.next().unwrap());
            let g2 = to_hex(chars.next().unwrap());

            let b1 = to_hex(chars.next().unwrap());
            let b2 = to_hex(chars.next().unwrap());

            Color {
                r: (r1 << 4) | r2,
                g: (g1 << 4) | g2,
                b: (b1 << 4) | b2,
            }
        }
    }
}
