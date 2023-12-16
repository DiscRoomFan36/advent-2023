use std::{collections::HashMap, fmt::Display, hash::Hash, iter::zip};

use rayon::prelude::*;

type IntType = u32;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

fn file_to_grid(file: &str) -> Vec<Vec<RockType>> {
    let grid: Vec<Vec<RockType>> = file
        .lines()
        .map(|line| line.chars().map(|char| RockType::new(char)).collect())
        .collect();
    let grid = transpose(grid);
    grid
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn tilt_west(grid: &mut Vec<Vec<RockType>>) {
    grid.par_iter_mut().for_each(|row| {
        row.split_mut(|rock| *rock == RockType::Cube)
            .for_each(|slice| slice.sort())
    })
}
fn tilt_east(grid: &mut Vec<Vec<RockType>>) {
    grid.par_iter_mut().for_each(|row| {
        row.split_mut(|rock| *rock == RockType::Cube)
            .for_each(|slice| slice.sort_by(|a, b| b.cmp(a)))
    })
}

fn tilt_north(grid: &mut Vec<Vec<RockType>>) {

    (0..grid[0].len()).for_each(|i| {
        
        // roll farwad and swap

        let mut j = 0;
        while j < grid.len() {
            let mut r_count = 0;
            let mut k = j;
            while k < grid.len() {
                match grid[k][i] {
                    RockType::Cube => {
                        break;
                    }
                    RockType::Rounded => {
                        r_count += 1;
                    }
                    _ => {}
                }
                k += 1;
            }
            for k in j..grid.len() {
                match grid[k][i] {
                    RockType::Cube => break,
                    _ => {
                        grid[k][i] = if r_count > 0 {
                            r_count -= 1;
                            RockType::Rounded
                        } else {
                            RockType::None
                        }
                    }
                }
            }
            j = k + 1;
        }
    })
}
fn tilt_south(grid: &mut Vec<Vec<RockType>>) {
    grid.reverse();
    tilt_north(grid);
    grid.reverse();
}

fn cycle_grid_vec(grid: &mut Vec<Vec<RockType>>) {
    tilt_west(grid);
    tilt_north(grid);
    tilt_east(grid);
    tilt_south(grid);
}

fn count_west_load(grid: &[Vec<RockType>]) -> IntType {
    grid.par_iter()
        .map(|row| {
            row.iter()
                .enumerate()
                .filter(|(_, rock)| **rock == RockType::Rounded)
                .map(|(i, _)| (row.len() - i) as IntType)
                .sum::<u32>()
        })
        .sum()
}

pub fn solve_part_1(file: &str) -> Option<IntType> {
    let mut grid = file_to_grid(file);
    tilt_west(&mut grid);
    Some(count_west_load(&grid))
}

pub fn solve_part_2(file: &str) -> Option<IntType> {
    const CYCLES: usize = 1_000_000_000;

    let mut grid = file_to_grid(file);

    let mut hashmap = HashMap::new();
    hashmap.insert(grid.clone(), 0);

    let mut i = 1;
    while i < CYCLES {
        // grid = cycle_grid_vec(grid);
        cycle_grid_vec(&mut grid);
        if hashmap.contains_key(&grid) {
            break;
        }
        hashmap.insert(grid.clone(), i);
        i += 1;
    }

    let j = hashmap[&grid];

    // cycle some amounts of time more
    let cycle_length = i - j;
    let extra_cycles = (CYCLES - j) % cycle_length;
    for _ in 0..extra_cycles {
        // grid = cycle_grid_vec(grid);
        cycle_grid_vec(&mut grid);
    }

    Some(count_west_load(&grid))
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
