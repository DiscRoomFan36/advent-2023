
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
pub mod enums {
	#[derive(Debug, Clone, Copy, PartialEq, Eq)]
	pub enum Direction {
		Up,
		Down,
		Left,
		Right,
	}

	#[derive(Debug, Clone, Copy, PartialEq, Eq)]
	pub enum CompassDirection {
		North,
		South,
		East,
		West,
	}
}
