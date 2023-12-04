use once_cell::sync::Lazy;
use regex::Regex;

const REGEX: &str = r"(\d+( |$))|(\|)";

fn card_matching(line: &str) -> u32 {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(REGEX).unwrap());
    let caps: Vec<u32> = RE.find_iter(line).map(|m| m.as_str().trim_end().parse().unwrap_or(0)).collect();

    let separator = caps.iter().position(|&x| x == 0).expect("0 element in array");
    let (winning, numbers) = (&caps[..separator], &caps[separator+1..]);

    numbers.iter().fold(0, |z, x| {
        z + if winning.contains(x) { 1 } else { 0 }
    })
}

pub fn solve_part_1(file: &str) -> Option<u32> {
    Some(file.lines().fold(0, |z, line| {
        z + (2 as u32).pow(card_matching(line)) / 2
    }))
}

pub fn solve_part_2(file: &str) -> Option<u32> {
    let scores: Vec<u32> = file.lines().map(|line| card_matching(line)).collect();
    let mut counts = vec![1; scores.len()];
    for (i, score) in scores.iter().enumerate() {
        for j in i+1..i+1+*score as usize {
            counts[j] = counts[j] + counts[i]
        }
    }
    Some(counts.iter().sum())
}

const DAY: u8 = 4;

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
        assert_eq!(solve_part_1(&content), Some(13))
    }

    #[test]
    fn solves_second_problem() {
        let content = inputs::get_file(DAY, InputType::Sample);
        assert_eq!(solve_part_2(&content), Some(30))
    }
}