use rayon::prelude::*;
use std::{collections::HashMap, fmt::Display, hash::Hash};

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
    grid
}

enum Tilt {
    North,
    West,
    South,
    East,
}

fn tilt_w_e(grid: &mut Vec<Vec<RockType>>, w_or_e: bool) {
    let (t, n_t) = if w_or_e {
        (RockType::Rounded, RockType::None)
    } else {
        (RockType::None, RockType::Rounded)
    };
    grid.par_iter_mut().for_each(|row| {
        let mut low = 0;
        let mut high = 0;
        while high < row.len() {
            match row[high] {
                RockType::Cube => {
                    low = high + 1;
                }
                x if x == t => {
                    if low != high {
                        row[high] = n_t;
                        row[low] = t;
                    }
                    low += 1;
                }
                _ => {}
            }
            high += 1;
        }
    })
}
fn tilt_n_s(grid: &mut Vec<Vec<RockType>>, n_or_s: bool) {
    let (t, n_t) = if n_or_s {
        (RockType::Rounded, RockType::None)
    } else {
        (RockType::None, RockType::Rounded)
    };
    (0..grid[0].len()).for_each(|i| {
        let mut low = 0;
        let mut high = 0;
        while high < grid.len() {
            match grid[high][i] {
                RockType::Cube => {
                    low = high + 1;
                }
                x if x == t => {
                    if low != high {
                        grid[high][i] = n_t;
                        grid[low][i] = t;
                    }
                    low += 1;
                }
                _ => {}
            }
            high += 1;
        }
    })
}
fn tilt(grid: &mut Vec<Vec<RockType>>, tilt: Tilt) {
    match tilt {
        Tilt::West => tilt_w_e(grid, true),
        Tilt::North => tilt_n_s(grid, true),
        Tilt::East => tilt_w_e(grid, false),
        Tilt::South => tilt_n_s(grid, false),
    }
}

const CYCLE: [Tilt; 4] = [Tilt::North, Tilt::West, Tilt::South, Tilt::East];
fn cycle_grid_vec(grid: &mut Vec<Vec<RockType>>) {
    for t in CYCLE {
        tilt(grid, t)
    }
}

fn count_load(grid: &[Vec<RockType>]) -> IntType {
    grid.par_iter()
        .enumerate()
        .map(|(j, row)| {
            row.iter()
                .filter(|rock| **rock == RockType::Rounded)
                .map(|_| (grid.len() - j) as IntType)
                .sum::<u32>()
        })
        .sum()
}

pub fn solve_part_1(file: &str) -> Option<IntType> {
    let mut grid = file_to_grid(file);
    tilt(&mut grid, Tilt::North);
    Some(count_load(&grid))
}

pub fn solve_part_2(file: &str) -> Option<IntType> {
    const CYCLES: usize = 1_000_000_000;
    let mut grid = file_to_grid(file);
    let mut hashmap = HashMap::new();
    hashmap.insert(grid.clone(), 0);

    let mut i = 1;
    while i < CYCLES {
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
        cycle_grid_vec(&mut grid);
    }

    Some(count_load(&grid))
}

const DAY: u8 = 14;

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
        assert_eq!(solve_part_1(&content), Some(136))
    }

    #[test]
    fn solves_second_problem() {
        let content = inputs::get_file(DAY, InputType::Sample);
        assert_eq!(solve_part_2(&content), Some(64))
    }
}
