use std::iter::zip;

pub fn is_prefix(needle: &str, haystack: &str) -> bool {
    if haystack.len() < needle.len() {
        return false;
    }
    zip(needle.chars(), haystack.chars()).all(|(n, h)| n == h)
}

fn combine_first_and_last(line: &str, prefixes: &Vec<&str>) -> u32 {
    let mut first_digit = 0;
    let mut last_digit = 0;

    for start in 0..line.len() {
        let pointer = &line[start..];

        if let Some((i, _)) = prefixes
            .iter()
            .enumerate()
            .map(|(i, x)| (i, is_prefix(x, pointer)))
            .find(|(_, x)| *x)
        {
            let digit = i % 9 + 1;

            if first_digit == 0 {
                first_digit = digit
            }
            last_digit = digit
        }
    }
    (first_digit * 10 + last_digit).try_into().unwrap()
}

const DIGITS_AS_WORDS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
const DIGITS: [&str; 9] = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];

fn accumulate(file: &String, prefixes: &Vec<&str>) -> u32 {
    file.lines().fold(0, |total, line| {
        total + combine_first_and_last(line, &prefixes)
    })
}

pub fn solve_part_1(file: &String) -> Option<u32> {
    Some(accumulate(file, &DIGITS.into()))
}

pub fn solve_part_2(file: &String) -> Option<u32> {
    let prefixes = [&DIGITS[..], &DIGITS_AS_WORDS[..]].concat();
    Some(accumulate(file, &prefixes))
}

pub fn main(file: &String) {
    println!("Solving Day 1");
    println!("  part 1: {:?}", solve_part_1(&file));
    println!("  part 2: {:?}", solve_part_2(&file));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inputs::{self, InputType};

    #[test]
    fn prefix_function_works() {
        assert!(self::is_prefix("Hello", "Hello, World!"));
        assert!(!self::is_prefix("a", "b"));
        assert!(!self::is_prefix("bigger than", "bigger"));
    }

    #[test]
    fn solves_part_1() {
        let content = inputs::get_file_part(1, InputType::Sample, 1);
        assert_eq!(solve_part_1(&content), Some(142))
    }

    #[test]
    fn solves_part_2() {
        let content = inputs::get_file_part(1, InputType::Sample, 2);
        assert_eq!(solve_part_2(&content), Some(281))
    }
}
