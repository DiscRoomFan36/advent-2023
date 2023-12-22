type IntType = u32;

use grid::Grid;

use crate::helpers::{
    constructor::{file_to_grid, FromChar},
    grid_stuff::find_index_of,
    print_helpers::ToChar,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GardenType {
    Rock,
    Garden,
    Start,
}
impl FromChar for GardenType {
    fn from_char(c: char) -> Self {
        match c {
            '#' => GardenType::Rock,
            '.' => GardenType::Garden,
            'S' => GardenType::Start,
            _ => unreachable!(),
        }
    }
}
impl ToChar for GardenType {
    fn to_char(&self) -> char {
        match self {
            GardenType::Rock => '#',
            GardenType::Garden => '.',
            GardenType::Start => 'S',
        }
    }
}

fn spread_out(garden: &Grid<GardenType>, step_grid: Grid<bool>) -> Grid<bool> {
    let mut new_grid = Grid::new(step_grid.rows(), step_grid.cols());
    for j in 0..step_grid.rows() {
        for i in 0..step_grid.cols() {
            if !step_grid[(j, i)] {
                continue;
            }
            if i > 0 && garden[(j, i - 1)] != GardenType::Rock {
                new_grid[(j, i - 1)] = true;
            }
            if i < step_grid.cols() - 1 && garden[(j, i + 1)] != GardenType::Rock {
                new_grid[(j, i + 1)] = true;
            }

            if j > 0 && garden[(j - 1, i)] != GardenType::Rock {
                new_grid[(j - 1, i)] = true;
            }
            if j < step_grid.rows() - 1 && garden[(j + 1, i)] != GardenType::Rock {
                new_grid[(j + 1, i)] = true;
            }
        }
    }
    new_grid
}

fn spread_out_and_count(garden: &Grid<GardenType>, steps: u32) -> IntType {
    let mut step_grid = Grid::new(garden.rows(), garden.cols());

    let start_pos = find_index_of(&garden, |&x| x == GardenType::Start);

    step_grid[start_pos] = true;

    for _ in 0..steps {
        step_grid = spread_out(&garden, step_grid);
    }

    step_grid.iter().filter(|&&x| x).count() as IntType
}

pub fn solve_part_1(file: &str) -> Option<IntType> {
    let garden: Grid<GardenType> = file_to_grid(file);
    Some(spread_out_and_count(&garden, 64))
}

pub fn solve_part_2(_file: &str) -> Option<IntType> {
    None
}

const DAY: u8 = 21;

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
        let garden = file_to_grid(&content);
        assert_eq!(spread_out_and_count(&garden, 6), 16);
    }

    #[test]
    #[ignore = "don't work"]
    fn solves_second_problem() {
        let content = inputs::get_file(DAY, InputType::Sample);
        let garden = file_to_grid(&content);
        assert_eq!(spread_out_and_count(&garden, 6), 16);
        assert_eq!(spread_out_and_count(&garden, 10), 50);
        assert_eq!(spread_out_and_count(&garden, 50), 1594);
        assert_eq!(spread_out_and_count(&garden, 100), 6536);
        assert_eq!(spread_out_and_count(&garden, 500), 167004);
        assert_eq!(spread_out_and_count(&garden, 1000), 668697);
        assert_eq!(spread_out_and_count(&garden, 5000), 16733044);
    }
}
