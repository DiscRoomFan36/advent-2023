use rayon::prelude::*;

use crate::helpers::constructor::{FromChar, file_to_grid, Grid};
use crate::helpers::enums::Direction;

type IntType = u32;
type Position = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MirrorType {
    None,
    ReflectForward,
    ReflectBack,
    SplitPipe,
    SplitDash,
}
impl FromChar for MirrorType {
    fn from_char(char: char) -> Self {
        match char {
            '.' => MirrorType::None,
            '/' => MirrorType::ReflectForward,
            '\\' => MirrorType::ReflectBack,
            '|' => MirrorType::SplitPipe,
            '-' => MirrorType::SplitDash,
            _ => panic!(),
        }
    }
}
impl MirrorType {
    fn reflected_to(self, dir: Direction) -> Vec<Direction> {
        match self {
            MirrorType::None => {
                vec![dir]
            }
            MirrorType::ReflectForward => match dir {
                Direction::Up => vec![Direction::Right],
                Direction::Down => vec![Direction::Left],
                Direction::Left => vec![Direction::Down],
                Direction::Right => vec![Direction::Up],
            },
            MirrorType::ReflectBack => match dir {
                Direction::Up => vec![Direction::Left],
                Direction::Down => vec![Direction::Right],
                Direction::Left => vec![Direction::Up],
                Direction::Right => vec![Direction::Down],
            },
            MirrorType::SplitPipe => match dir {
                Direction::Up => vec![Direction::Up],
                Direction::Down => vec![Direction::Down],
                Direction::Left => vec![Direction::Up, Direction::Down],
                Direction::Right => vec![Direction::Up, Direction::Down],
            },
            MirrorType::SplitDash => match dir {
                Direction::Up => vec![Direction::Left, Direction::Right],
                Direction::Down => vec![Direction::Left, Direction::Right],
                Direction::Left => vec![Direction::Left],
                Direction::Right => vec![Direction::Right],
            },
        }
    }
}


// moves the beam forward, return None if off the edge
fn next_position(grid: &Grid<MirrorType>, (j, i): Position, dir: Direction) -> Option<Position> {
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

fn next_checks(
    grid: &Grid<MirrorType>,
    (pos, dir): (Position, Direction),
) -> Vec<(Position, Direction)> {
    grid[pos]
        .reflected_to(dir)
        .iter()
        .map(|&next_dir| next_position(grid, pos, next_dir).and_then(|pos| Some((pos, next_dir))))
        .filter_map(|x| x)
        .collect()
}

fn calculate_energized(mirrors: &Grid<MirrorType>, start: (Position, Direction)) -> IntType {
    let mut energy_grid: Grid<Vec<Direction>> = Grid::new(mirrors.rows(), mirrors.cols());
    let mut light_stack: Vec<(Position, Direction)> = vec![start];
    while let Some((pos, dir)) = light_stack.pop() {
        if !energy_grid[pos].contains(&dir) {
            energy_grid[pos].push(dir);
            light_stack.append(&mut next_checks(&mirrors, (pos, dir)))
        }
    }
    energy_grid.iter().filter(|dirs| !dirs.is_empty()).count() as IntType
}

pub fn solve_part_1(file: &str) -> Option<IntType> {
    Some(calculate_energized(
        &file_to_grid(file),
        ((0, 0), Direction::Right),
    ))
}

pub fn solve_part_2(file: &str) -> Option<IntType> {
    let mirrors = file_to_grid(file);

    assert_eq!(mirrors.rows(), mirrors.cols());
    let starts: Vec<(Position, Direction)> = (0..mirrors.rows())
        .flat_map(|i| {
            vec![
                ((i, 0), Direction::Right),
                ((i, mirrors.rows() - 1), Direction::Left),
                ((0, i), Direction::Down),
                ((mirrors.rows() - 1, i), Direction::Up),
            ]
        })
        .collect();

    Some(
        starts
            .par_iter()
            .map(|start| calculate_energized(&mirrors, *start))
            .max()
            .unwrap(),
    )
}

const DAY: u8 = 16;

#[allow(unused)]
pub fn main(file: &str) {
    println!("Solving Day {}", DAY);
    println!("  part 1: {:?}", solve_part_1(&file));
    println!("  part 2: {:?}", solve_part_2(&file));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inputs::{self, InputType};

    #[test]
    fn solves_first_problem() {
        let content = inputs::get_file(DAY, InputType::Sample);
        assert_eq!(solve_part_1(&content), Some(46))
    }

    #[test]
    fn solves_second_problem() {
        let content = inputs::get_file(DAY, InputType::Sample);
        assert_eq!(solve_part_2(&content), Some(51))
    }
}
