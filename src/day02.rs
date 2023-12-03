use once_cell::sync::Lazy;
use regex::Regex;
use std::{cmp::max, iter::zip};

const GAME_ID_CAPTURE: &str = r"^Game (?P<id>\d+)";
const NUM_COLOR_CAPTURE: &str = r"(?P<num>\d+) (?P<color>\w*)";

fn get_game_id(line: &str) -> u32 {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(GAME_ID_CAPTURE).unwrap());
    let caps = RE.captures(line).unwrap();
    caps["id"].parse().unwrap()
}

fn smallest_hands(hand: &str) -> [u32; 3] {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(NUM_COLOR_CAPTURE).unwrap());
    RE.captures_iter(hand)
        .map(|c| c.extract())
        .fold([0, 0, 0], |z, (_, [num, color])| {
            let [r, g, b] = z;
            let num: u32 = num.parse().unwrap();
            match color {
                "red" => [max(r, num), g, b],
                "green" => [r, max(g, num), b],
                "blue" => [r, g, max(b, num)],
                _ => panic!(),
            }
        })
}

pub fn solve_part_1(file: &String) -> Option<u32> {
    const MAX_HANDS: [u32; 3] = [12, 13, 14];
    Some(file.lines().fold(0, |z, line: &str| {
        let hand = smallest_hands(&line);
        if zip(hand, MAX_HANDS).all(|(h, m)| h <= m) {
            z + get_game_id(line)
        } else {
            z
        }
    }))
}

pub fn solve_part_2(file: &String) -> Option<u32> {
    Some(file.lines().fold(0, |z, line| {
        z + smallest_hands(&line).iter().fold(1, |z, u| z * u)
    }))
}

pub fn main(file: &String) {
    println!("Solving Day 2");
    println!("  part 1: {:?}", solve_part_1(&file));
    println!("  part 2: {:?}", solve_part_2(&file));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inputs::{self, InputType};

    const DAY: u8 = 2;

    #[test]
    fn regex_captures_game_id() {
        let input = "Game 100: 8 red, 2 blue, 1 green; 2 blue, 4 red, 2 green; 9 red, 1 green; 2 green, 2 red; 3 red, 5 blue; 5 blue, 8 red";
        let re = Regex::new(GAME_ID_CAPTURE).unwrap();
        let caps = re.captures(input).unwrap();
        assert_eq!(caps["id"].parse::<u32>().unwrap(), 100);
    }

    #[test]
    fn solves_part_1() {
        let content = inputs::get_file(DAY, InputType::Sample);
        assert_eq!(solve_part_1(&content), Some(8))
    }

    #[test]
    fn solves_part_2() {
        let content = inputs::get_file(DAY, InputType::Sample);
        assert_eq!(solve_part_2(&content), Some(2286))
    }
}
