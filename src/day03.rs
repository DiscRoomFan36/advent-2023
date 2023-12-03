use std::{char, cmp::min, usize};

fn to_grid(file: &String) -> Vec<Vec<u8>> {
    file.lines().map(|line| line.as_bytes().into()).collect()
}

fn is_symbol(c: u8) -> bool {
    match c {
        b'.' => false,
        b'0'..=b'9' => false,
        _ => true,
    }
}

fn is_gear(c: u8) -> bool {
    match c {
        b'*' => true,
        _ => false,
    }
}

// takes a pos with a number and finds the start and end to construct a number
fn to_number(grid: &Vec<Vec<u8>>, pos: (usize, usize)) -> u32 {
    let (x, y) = pos;

    assert!(grid[y][x].is_ascii_digit());

    let start_x = (0..=x)
        .rev()
        .take_while(|x| grid[y][*x].is_ascii_digit())
        .last()
        .unwrap();

    grid[y][start_x..]
        .iter()
        .take_while(|x| x.is_ascii_digit())
        .fold(0, |z, u| z * 10 + (*u as char).to_digit(10).unwrap())
}

fn surrounding_numbers(grid: &Vec<Vec<u8>>, pos: (usize, usize)) -> Vec<u32> {
    let (x, y) = pos;
    let mut numbers = Vec::new();
    for j in y.checked_sub(1).unwrap_or(0)..=min(y + 1, grid.len()) {
        for i in x.checked_sub(1).unwrap_or(0)..=min(x + 1, grid.len()) {
            if grid[j][i].is_ascii_digit() {
                // do some duplicate checking
                if i > 0 && i >= x && grid[j][i - 1].is_ascii_digit() {
                    continue;
                }
                numbers.push(to_number(grid, (i, j)));
            }
        }
    }
    numbers
}

pub fn solve_part_1(file: &String) -> Option<u32> {
    let mut total = 0;
    let grid = to_grid(file);
    // its a square
    assert_eq!(grid.len(), grid[0].len());

    for j in 0..grid.len() {
        for i in 0..grid.len() {
            if is_symbol(grid[j][i]) {
                total += surrounding_numbers(&grid, (i, j)).iter().sum::<u32>();
            }
        }
    }
    Some(total)
}

pub fn solve_part_2(file: &String) -> Option<u32> {
    let mut total = 0;
    let grid = to_grid(file);
    // its a square
    assert_eq!(grid.len(), grid[0].len());

    for j in 0..grid.len() {
        for i in 0..grid.len() {
            if is_gear(grid[j][i]) {
                let numbs = surrounding_numbers(&grid, (i, j));
                if numbs.len() == 2 {
                    total = total + numbs.iter().product::<u32>()
                }
            }
        }
    }
    Some(total)
}

const DAY: u8 = 3;

pub fn main(file: &String) {
    println!("Solving Day {}", DAY);
    println!("  part 1: {:?}", solve_part_1(&file));
    println!("  part 2: {:?}", solve_part_2(&file));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inputs::{self, InputType};

    #[test]
    fn detects_symbol() {
        assert!(!is_symbol(b'.'));
        assert!(!is_symbol(b'0'));
        assert!(!is_symbol(b'5'));
        assert!(!is_symbol(b'9'));
        assert!(is_symbol(b'*'));
        assert!(is_symbol(b'/'));
        assert!(is_symbol(b'&'));
        assert!(is_symbol(b'$'));
    }

    #[test]
    fn solves_part_1() {
        let content = inputs::get_file(DAY, InputType::Sample);
        assert_eq!(solve_part_1(&content), Some(4361))
    }

    #[test]
    fn solves_part_2() {
        let content = inputs::get_file(DAY, InputType::Sample);
        assert_eq!(solve_part_2(&content), Some(467835))
    }
}
