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
pub mod array_helpers {

    pub fn contains_only<T: PartialEq>(array: &[T], only: &[T]) -> bool {
        for item in array {
            if !only.contains(item) {
                return false;
            }
        }
        true
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

    use std::collections::VecDeque;

    pub fn flood_fill_count<F>(
        size: (usize, usize),
        pos: (usize, usize),
        count: usize,
        f: F,
    ) -> Grid<usize>
    where
        F: Fn((usize, usize)) -> bool,
    {
        let (rows, cols) = size;
        let mut flooding_grid = Grid::new(rows, cols);
        let mut flood_stack = VecDeque::new();
        flood_stack.push_back((pos, count));

        while let Some(((y, x), c)) = flood_stack.pop_front() {
            flooding_grid[(y, x)] = c;

            if x > 0 && (flooding_grid[(y, x - 1)] < c - 1 && f((y, x - 1))) {
                flood_stack.push_back(((y, x - 1), c - 1));
            }
            if x < cols - 1 && (flooding_grid[(y, x + 1)] < c - 1 && f((y, x + 1))) {
                flood_stack.push_back(((y, x + 1), c - 1));
            }

            if y > 0 && (flooding_grid[(y - 1, x)] < c - 1 && f((y - 1, x))) {
                flood_stack.push_back(((y - 1, x), c - 1));
            }
            if y < rows - 1 && (flooding_grid[(y + 1, x)] < c - 1 && f((y + 1, x))) {
                flood_stack.push_back(((y + 1, x), c - 1));
            }
        }

        flooding_grid
    }

    fn grid_to_bool_grid<T, F>(grid: Grid<T>, f: F) -> Grid<bool>
    where
        F: Fn(&T) -> bool,
    {
        let (rows, cols) = grid.size();
        let mut new_grid = Grid::new(rows, cols);

        for j in 0..rows {
            for i in 0..cols {
                new_grid[(j, i)] = f(&grid[(j, i)]);
            }
        }

        new_grid
    }

    pub fn find_index_of<T, F>(grid: &Grid<T>, f: F) -> (usize, usize)
    where
        F: Fn(&T) -> bool,
    {
        for j in 0..grid.rows() {
            for i in 0..grid.cols() {
                if f(&grid[(j, i)]) == true {
                    return (j, i);
                }
            }
        }
        panic!()
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

    impl ToChar for usize {
        fn to_char(&self) -> char {
            match self {
                0 => '0',
                1 => '1',
                2 => '2',
                3 => '3',
                4 => '4',
                5 => '5',
                6 => '6',
                7 => '7',
                8 => '8',
                9 => '9',
                _ => '#',
            }
        }
    }

    impl<T> ToChar for Option<T>
    where
        T: ToChar,
    {
        fn to_char(&self) -> char {
            if let Some(x) = self {
                x.to_char()
            } else {
                '.'
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

pub mod math {
    pub fn gcd<T>(mut a: T, mut b: T) -> T
    where
        T: std::ops::Rem<Output = T> + Default + std::cmp::PartialEq + Copy,
    {
        while b != T::default() {
            (a, b) = (b, a % b);
        }
        a
    }
    pub fn lcm<T>(a: T, b: T) -> T
    where
        T: std::ops::Rem<Output = T>
            + std::ops::Div<Output = T>
            + std::ops::Mul<Output = T>
            + Default
            + std::cmp::PartialEq
            + Copy,
    {
        a / gcd(a, b) * b
    }
}
