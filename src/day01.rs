use std::iter::zip;

pub fn is_prefix(needle: &str, haystack: &str) -> bool {
    if haystack.len() < needle.len() {
        return false;
    }
    zip(needle.chars(), haystack.chars()).all(|(n, h)| n == h)
}

fn combine_first_and_last(line: &String, prefixes: &Vec<&str>) -> u32 {
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

fn accumulate(file: &Vec<String>, prefixes: &Vec<&str>) -> u32 {
    file.iter().fold(0, |total, line| {
        total + combine_first_and_last(line, &prefixes)
    })
}

pub fn solve_part_1(file: &Vec<String>) -> u32 {
    accumulate(file, &DIGITS.into())
}

pub fn solve_part_2(file: &Vec<String>) -> u32 {
    accumulate(file, &[&DIGITS[..], &DIGITS_AS_WORDS[..]].concat())
}

pub fn main(file: &Vec<String>) {
    println!("Solving Day 1");
    let answer = solve_part_1(&file);
    println!("answer 1 is {answer}");
    let answer = solve_part_2(&file);
    println!("answer 2 is {answer}");
}


#[cfg(test)]
mod tests {
    use crate::inputs::{self, InputType};
    use super::*;

    #[test]
    fn prefix_function_works() {
        assert!(self::is_prefix("Hello", "Hello, World!"));
        assert!(!self::is_prefix("a", "b"));
        assert!(!self::is_prefix("bigger than", "bigger"));
    }

    #[test]
    fn solves_first_problem() {
        let content = inputs::load_file(1, InputType::Sample(1));
        assert_eq!(solve_part_1(&content), 142)
    }

    #[test]
    fn solves_second_problem() {
        let content = inputs::load_file(1, InputType::Sample(2));
        assert_eq!(solve_part_2(&content), 281)
    }
}
