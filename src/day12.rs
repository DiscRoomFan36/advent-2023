use std::collections::HashMap;

use rayon::prelude::*;

use once_cell::sync::Lazy;
use regex::Regex;

type IntType = u64;

fn line_to_digits(line: &str) -> Vec<u8> {
    const REGEX: &str = r"(-?\d+)";
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(REGEX).unwrap());
    RE.find_iter(line)
        .map(|m| m.as_str().trim_end().parse().unwrap_or(0))
        .collect()
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum State {
    Operational,
    Damaged,
    Unknown,
}
impl State {
    fn new(char: char) -> Self {
        match char {
            '.' => Self::Operational,
            '#' => Self::Damaged,
            '?' => Self::Unknown,
            _ => panic!(),
        }
    }
}

fn recur_all_permutations(
    n: u8,
    m: u8,
    run: u8,
    states: &[State],
    run_lens: &[u8],
    hashmap: &mut HashMap<[u8; 4], IntType>,
) -> IntType {
    if hashmap.contains_key(&[n, m, run, run_lens.len() as u8]) {
        hashmap[&[n, m, run, run_lens.len() as u8]]
    } else {
        let answer = if states.len() == 0 {
            if run > 0 {
                if run_lens.len() == 0 || run != run_lens[0] {
                    0
                } else {
                    if run_lens.len() == 1 {
                        1
                    } else {
                        0
                    }
                }
            } else {
                if run_lens.len() == 0 {
                    1
                } else {
                    0
                }
            }
        } else {
            match states[0] {
                State::Operational => {
                    if run > 0 {
                        if run_lens.len() == 0 || run != run_lens[0] {
                            0
                        } else {
                            recur_all_permutations(n, m, 0, &states[1..], &run_lens[1..], hashmap)
                        }
                    } else {
                        recur_all_permutations(n, m, 0, &states[1..], run_lens, hashmap)
                    }
                }
                State::Damaged => {
                    if run_lens.len() == 0 || run + 1 > run_lens[0] {
                        0
                    } else {
                        recur_all_permutations(n, m, run + 1, &states[1..], run_lens, hashmap)
                    }
                }
                State::Unknown => {
                    let yes = if m > 0 {
                        if run < run_lens[0] {
                            recur_all_permutations(
                                n - 1,
                                m - 1,
                                run + 1,
                                &states[1..],
                                run_lens,
                                hashmap,
                            )
                        } else {
                            0
                        }
                    } else {
                        0
                    };

                    let no = if run > 0 {
                        if run == run_lens[0] {
                            recur_all_permutations(
                                n - 1,
                                m,
                                0,
                                &states[1..],
                                &run_lens[1..],
                                hashmap,
                            )
                        } else {
                            0
                        }
                    } else {
                        recur_all_permutations(n - 1, m, 0, &states[1..], run_lens, hashmap)
                    };

                    yes + no
                }
            }
        };
        hashmap.insert([n, m, run, run_lens.len() as u8], answer);
        answer
    }
}

struct Record {
    states: Vec<State>,
    numbers: Vec<u8>,
}

impl Record {
    fn line_to_record(line: &str) -> Self {
        let (line, numbers) = line.split_once(" ").unwrap();
        let states = line.chars().map(|byte| State::new(byte)).collect();
        let numbers = line_to_digits(numbers);

        Record { states, numbers }
    }
    fn num_unknown(&self) -> usize {
        self.states
            .iter()
            .filter(|state| **state == State::Unknown)
            .count()
    }
    fn number_of_arrangements(&self) -> IntType {
        let n = self.num_unknown() as u8;
        let m = self.numbers.iter().sum::<u8>()
            - self.states.iter().filter(|x| **x == State::Damaged).count() as u8;

        recur_all_permutations(n, m, 0, &self.states, &self.numbers, &mut HashMap::new())
    }
}

pub fn solve_part_1(file: &str) -> Option<IntType> {
    Some(
        file.par_lines()
            .map(|line| Record::line_to_record(line))
            .map(|record| Record::number_of_arrangements(&record))
            .sum(),
    )
}

pub fn solve_part_2(file: &str) -> Option<IntType> {
    Some(
        file.par_lines()
            .map(|line| Record::line_to_record(line))
            .map(|record| Record {
                states: (0..4).fold(record.states.clone(), |z, _| {
                    [z.clone(), vec![State::Unknown], record.states.clone()].concat()
                }),
                numbers: record
                    .numbers
                    .iter()
                    .cycle()
                    .take(5 * record.numbers.len())
                    .map(|x| *x)
                    .collect(),
            })
            .map(|record| Record::number_of_arrangements(&record))
            .sum(),
    )
}

const DAY: u8 = 12;

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
        assert_eq!(solve_part_1(&content), Some(21))
    }

    #[test]
    fn solves_second_problem() {
        let content = inputs::get_file(DAY, InputType::Sample);
        assert_eq!(solve_part_2(&content), Some(525152))
    }
}
