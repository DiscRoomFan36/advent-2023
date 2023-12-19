use crate::helpers::{
    color::hex_to_bin, enums_and_types::Direction, positions_and_directions::next_position_counted,
};
use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DigStep {
    dir: Direction,
    count: usize,
}
impl DigStep {
    fn line_to_step(line: &str, correct: bool) -> Self {
        const REGEX: &str =
            r"(?P<dir>\w) (?P<count>\d+) \(#(?P<count2>[[:xdigit:]]{5})(?P<dir2>[[:xdigit:]])\)";
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(REGEX).unwrap());
        let (_, [dir, count, count_true, dir_true]) = RE.captures(line).unwrap().extract();

        match correct {
            false => DigStep {
                dir: match dir {
                    "U" => Direction::Up,
                    "D" => Direction::Down,
                    "R" => Direction::Right,
                    "L" => Direction::Left,
                    _ => panic!(),
                },
                count: count.parse().unwrap(),
            },
            true => DigStep {
                dir: match dir_true {
                    "0" => Direction::Right,
                    "1" => Direction::Down,
                    "2" => Direction::Left,
                    "3" => Direction::Up,
                    _ => panic!(),
                },
                count: hex_to_bin(count_true),
            },
        }
    }
    fn file_to_steps(file: &str, correct: bool) -> Vec<Self> {
        file.lines()
            .map(|line| DigStep::line_to_step(line, correct))
            .collect()
    }
}

type IntType = i64;

fn get_interior_volume(steps: Vec<DigStep>) -> IntType {
    // use polygon area formula, inputs to big
    let mut area: i64 = 0;
    let mut line: i64 = 0;
    let (mut row, mut col): (i64, i64) = (0, 0);
    for i in 0..steps.len() {
        let item = steps[i];
        let (next_row, next_col) = next_position_counted((row, col), item.dir, item.count as i64);

        area += (col * next_row) - (row * next_col);
        line += item.count as i64;

        (row, col) = (next_row, next_col);
    }

    (area / 2 + line / 2 + 1) as IntType
}

pub fn solve_part_1(file: &str) -> Option<IntType> {
    let steps = DigStep::file_to_steps(file, false);
    Some(get_interior_volume(steps))
}

pub fn solve_part_2(file: &str) -> Option<IntType> {
    let steps = DigStep::file_to_steps(file, true);
    Some(get_interior_volume(steps))
}

const DAY: u8 = 18;

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
        assert_eq!(solve_part_1(&content), Some(62))
    }

    #[test]
    fn solves_second_problem() {
        let content = inputs::get_file(DAY, InputType::Sample);
        assert_eq!(solve_part_2(&content), Some(952408144115))
    }
}
