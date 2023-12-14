use std::{fmt::Display, collections::HashMap, hash::{self, Hash}};

use grid::Grid;

type IntType = u32;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum RockType {
    Rounded,
    Cube,
    None,
}
impl RockType {
    fn new(char: char) -> Self {
        match char {
            'O' => RockType::Rounded,
            '#' => RockType::Cube,
            '.' => RockType::None,
            _ => panic!(),
        }
    }
}
impl Display for RockType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                RockType::Rounded => 'O',
                RockType::Cube => '#',
                RockType::None => '.',
            }
        )
    }
}

fn file_to_grid(file: &str) -> Grid<RockType> {
    let grid: Vec<Vec<RockType>> = file
        .lines()
        .map(|line| line.chars().map(|char| RockType::new(char)).collect())
        .collect();
    let mut grid = Grid::from_vec_with_order(
        grid.iter().map(|x| x.clone()).flatten().collect(),
        grid[0].len(),
        grid::Order::ColumnMajor,
    );
    grid.transpose();
    grid
}

fn move_col_up(grid: &mut Grid<RockType>, col_index: usize) {

    for i in 1..grid.rows() {
        if grid[(i, col_index)] == RockType::Rounded {
            // move up
            let mut j = i;
            while j > 0 {
                j -= 1;
                if grid[(j, col_index)] != RockType::None {
                    j += 1;
                    break;
                }
            }
            grid[(i, col_index)] = RockType::None;
            grid[(j, col_index)] = RockType::Rounded;
        }
    }
}

use rayon::prelude::*;

fn tilt_north(grid: &mut Grid<RockType>) {
    // grid.iter_col_mut(col)
    // (0..grid.cols()).into_par_iter().for_each(|col_index| {
    //     move_col_up(grid, col_index)
    // });

    for col_index in 0..grid.cols() {

        // grid.ite

        for i in 1..grid.rows() {
            if grid[(i, col_index)] == RockType::Rounded {
                // move up
                let mut j = i;
                while j > 0 {
                    j -= 1;
                    if grid[(j, col_index)] != RockType::None {
                        j += 1;
                        break;
                    }
                }
                grid[(i, col_index)] = RockType::None;
                grid[(j, col_index)] = RockType::Rounded;
            }
        }

        // move_col_up(grid, col_index);
    }
}

fn count_weight(grid: &Grid<RockType>) -> IntType {
    let mut count = 0; 
    for j in 0..grid.rows() {
        for i in 0..grid.cols() {
            if grid[(i, j)] == RockType::Rounded {
                count += (grid.rows() - i) as IntType;
            }
        }
    }
    count
}

fn cycle(grid: &mut Grid<RockType>) {
    for _ in 0..4 {
        tilt_north(grid);
        grid.rotate_right();
    }
}

pub fn solve_part_1(file: &str) -> Option<IntType> {
    let mut grid = file_to_grid(file);
    tilt_north(&mut grid);
    Some(count_weight(&grid))
}

pub fn solve_part_2(file: &str) -> Option<IntType> {
    const CYCLES: usize = 1_000_000_000;

    let mut grid = file_to_grid(file);

    // i wish grid had a hash
    let mut hashmap = HashMap::new();
    hashmap.insert(grid.flatten().clone(), 0);

    let mut i = 1;
    while i < CYCLES {
        cycle(&mut grid);
        if hashmap.contains_key(grid.flatten()) {
            break;
        }
        hashmap.insert(grid.flatten().clone(), i);
        i += 1;
    }

    let j = hashmap[grid.flatten()];

    // cycle some amounts of time more
    let cycle_length = i-j;
    let extra_cycles =  (CYCLES - j) % cycle_length;
    for _ in 0..extra_cycles {
        cycle(&mut grid);
    }
    
    Some(count_weight(&grid))
}

const DAY: u8 = 14;

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
        assert_eq!(solve_part_1(&content), Some(136))
    }

    #[test]
    fn solves_second_problem() {
        let content = inputs::get_file(DAY, InputType::Sample);
        assert_eq!(solve_part_2(&content), Some(64))
    }
}
