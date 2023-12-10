use once_cell::sync::Lazy;
use regex::Regex;

type IntType = i32;

const REGEX: &str = r"(-?\d+)";
fn line_to_digits(line: &str) -> Vec<IntType> {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(REGEX).unwrap());
    RE.find_iter(line)
        .map(|m| m.as_str().trim_end().parse().unwrap_or(0))
        .collect()
}

type Readings = Vec<IntType>;

fn extrapolate_readings(readings: Readings) -> Vec<Readings> {
    let mut stack: Vec<Readings> = vec![readings];
    while !stack.last().unwrap().iter().all(|&reading| reading == 0) {
        let prev = stack.last().unwrap();
        let next: Readings = (0..prev.len() - 1).map(|i| prev[i + 1] - prev[i]).collect();
        stack.push(next);
    }
    stack
}

fn next_data_point(readings: Readings) -> IntType {
    let mut stack = extrapolate_readings(readings);

    stack.last_mut().unwrap().push(0);
    (0..stack.len() - 1).rev().for_each(|i| {
        let next_reading = stack[i].last().unwrap() + stack[i + 1].last().unwrap();
        stack[i].push(next_reading)
    });
    *stack[0].last().unwrap()
}

pub fn solve_part_1(file: &str) -> Option<IntType> {
    Some(
        file.lines()
            .map(|line| line_to_digits(line))
            .map(|readings| next_data_point(readings))
            .sum(),
    )
}

pub fn solve_part_2(file: &str) -> Option<IntType> {
    // just reverse the list, how hard could it be?
    Some(
        file.lines()
            .map(|line| line_to_digits(line))
            .map(|x| x.iter().rev().map(|x| *x).collect::<Vec<i32>>())
            .map(|readings| next_data_point(readings))
            .sum(),
    )
}

const DAY: u8 = 9;

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
        assert_eq!(solve_part_1(&content), Some(114))
    }

    #[test]
    fn solves_second_problem() {
        let content = inputs::get_file(DAY, InputType::Sample);
        assert_eq!(solve_part_2(&content), Some(2))
    }
}
