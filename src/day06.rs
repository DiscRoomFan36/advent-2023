use std::iter::zip;

use once_cell::sync::Lazy;
use regex::Regex;

type IntType = u64;

const REGEX: &str = r"(\d+)";
fn line_to_digits(line: &str) -> Vec<IntType> {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(REGEX).unwrap());
    RE.find_iter(line)
        .map(|m| m.as_str().trim_end().parse().unwrap_or(0))
        .collect()
}

fn pressed_to_distance(total_time: IntType, pressed: IntType) -> IntType {
    total_time.checked_sub(pressed).unwrap_or_default() * pressed
}

fn how_many_ways(time: IntType, dist: IntType) -> IntType {
    let (mut low, mut high) = (0, time / 2);
    while low != high {
        let mid = low + (high - low) / 2;
        if pressed_to_distance(time, mid) < dist {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    let smallest = low;

    let (mut low, mut high) = (time / 2, time);
    while low != high {
        let mid = low + (high - low) / 2;
        if pressed_to_distance(time, mid) >= dist {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    let largest = low - 1;

    let smallest = if pressed_to_distance(time, smallest) > dist {
        smallest
    } else {
        smallest + 1
    };
    let largest = if pressed_to_distance(time, largest) > dist {
        largest
    } else {
        largest - 1
    };

    largest - smallest + 1
}

pub fn solve_part_1(file: &str) -> Option<IntType> {
    let inputs: Vec<Vec<IntType>> = file.lines().map(|line| line_to_digits(line)).collect();
    let (times, distances) = (inputs[0].clone(), inputs[1].clone());

    Some(zip(times, distances).fold(1, |z, (time, dist)| z * how_many_ways(time, dist)))
}

pub fn solve_part_2(file: &str) -> Option<IntType> {
    let inputs: Vec<IntType> = file
        .lines()
        .map(|line| {
            line.split_once(":")
                .unwrap()
                .1
                .replace(" ", "")
                .parse()
                .unwrap()
        })
        .collect();
    let (time, dist) = (inputs[0], inputs[1]);
    Some(how_many_ways(time, dist))
}

const DAY: u8 = 6;

#[allow(unused)]
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
    fn solves_first_problem() {
        let content = inputs::get_file(DAY, InputType::Sample);
        assert_eq!(solve_part_1(&content), Some(288))
    }

    #[test]
    fn solves_second_problem() {
        let content = inputs::get_file(DAY, InputType::Sample);
        assert_eq!(solve_part_2(&content), Some(71503))
    }
}
