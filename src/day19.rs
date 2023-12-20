use once_cell::sync::Lazy;
use regex::Regex;

use std::{
    cmp::{max, min},
    collections::HashMap,
};

use crate::helpers::constructor::FromChar;

type RangeInt = u16;
type IntType = u64;

#[derive(Debug, Clone, Copy)]
enum Property {
    X,
    M,
    A,
    S,
}
impl FromChar for Property {
    fn from_char(c: char) -> Self {
        match c {
            'x' => Property::X,
            'm' => Property::M,
            'a' => Property::A,
            's' => Property::S,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone)]
struct Instruction {
    property: Property,
    is_lesser: bool,
    number: RangeInt,
    output: String,
}

#[derive(Debug, Clone)]
struct WorkFlow {
    ident: String,
    instructions: Vec<Instruction>,
    otherwise: String,
}
impl WorkFlow {
    fn new(line: &str) -> Self {
        const START_AND_END: &str = r"(?P<ident>\w+)\{.*,(?P<end_ident>\w+)\}";
        static RE_S_A_E: Lazy<Regex> = Lazy::new(|| Regex::new(START_AND_END).unwrap());
        let (_, [ident, end_ident]) = RE_S_A_E.captures(line).unwrap().extract();

        const INSTRUCTIONS: &str = r"(?P<part>[xmas])(?P<cmp>[<>])(?P<num>\d+):(?P<end>\w+)";
        static RE_INS: Lazy<Regex> = Lazy::new(|| Regex::new(INSTRUCTIONS).unwrap());
        let instructions = RE_INS
            .captures_iter(line)
            .map(|c| c.extract())
            .map(|(_, [part, cmp, num, end])| Instruction {
                property: Property::from_char(part.chars().last().unwrap()),
                is_lesser: match cmp {
                    "<" => true,
                    ">" => false,
                    _ => panic!(),
                },
                number: num.parse().unwrap(),
                output: end.to_string(),
            })
            .collect();

        WorkFlow {
            ident: ident.to_string(),
            otherwise: end_ident.to_string(),
            instructions,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Part {
    x: RangeInt,
    m: RangeInt,
    a: RangeInt,
    s: RangeInt,
}
impl Part {
    fn new(line: &str) -> Self {
        const REGEX_PART: &str = r"\{x=(?P<x>\d+),m=(?P<m>\d+),a=(?P<a>\d+),s=(?P<s>\d+)\}";
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(REGEX_PART).unwrap());
        let (_, [x, m, a, s]) = RE.captures(line).unwrap().extract();
        Part {
            x: x.parse().unwrap(),
            m: m.parse().unwrap(),
            s: s.parse().unwrap(),
            a: a.parse().unwrap(),
        }
    }
    fn get_total(&self) -> RangeInt {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug, Clone, Copy)]
struct Range {
    start: RangeInt,
    end: RangeInt,
}
impl Range {
    fn new(start: RangeInt, end: RangeInt) -> Self {
        Range { start, end }
    }
}

#[derive(Debug, Clone, Copy)]
struct Ranges {
    x: Range,
    m: Range,
    a: Range,
    s: Range,
}
impl Ranges {
    fn get_part(&self, property: Property) -> Range {
        match property {
            Property::X => self.x,
            Property::M => self.m,
            Property::A => self.a,
            Property::S => self.s,
        }
    }
    fn get_part_mut(&mut self, property: Property) -> &mut Range {
        match property {
            Property::X => &mut self.x,
            Property::M => &mut self.m,
            Property::A => &mut self.a,
            Property::S => &mut self.s,
        }
    }
    fn replace_part(mut self, property: Property, new_r: Range) -> Self {
        *self.get_part_mut(property) = new_r;
        self
    }
    fn number_permutations(&self) -> IntType {
        const PROPERTIES: [Property; 4] = [Property::X, Property::M, Property::A, Property::S];
        PROPERTIES.iter().fold(1, |z, &u| {
            let r = self.get_part(u);
            z * (r.end - r.start) as IntType
        })
    }
}

fn parse_file(file: &str) -> (HashMap<String, WorkFlow>, Vec<Part>) {
    let (works, parts) = file.split_once("\n\n").unwrap();

    let work_flows: Vec<WorkFlow> = works.lines().map(|line| WorkFlow::new(line)).collect();
    let mut hashmap: HashMap<String, WorkFlow> = HashMap::new();
    for work_flow in work_flows {
        hashmap.insert(work_flow.ident.clone(), work_flow.clone());
    }

    let parts = parts.lines().map(|line| Part::new(line)).collect();

    (hashmap, parts)
}

fn recur_stack(hashmap: &HashMap<String, WorkFlow>, ranges: Ranges) -> IntType {
    let mut total = 0;

    let mut stack = vec![("in", ranges)];
    'recur: while let Some((ident, mut ranges)) = stack.pop() {
        let work_flow = match ident {
            "R" => continue 'recur,
            "A" => {
                total += ranges.number_permutations();
                continue 'recur;
            }
            ident => &hashmap[ident],
        };

        for ins in work_flow.instructions.iter() {
            let r = ranges.get_part(ins.property);
            if ins.is_lesser {
                if r.start < ins.number {
                    stack.push((
                        &ins.output,
                        ranges.replace_part(
                            ins.property,
                            Range::new(r.start, min(r.end, ins.number)),
                        ),
                    ));
                }
                if r.end >= ins.number {
                    *ranges.get_part_mut(ins.property) = Range::new(max(r.start, ins.number), r.end)
                } else {
                    continue 'recur;
                }
            } else {
                if r.end > ins.number {
                    stack.push((
                        &ins.output,
                        ranges.replace_part(
                            ins.property,
                            Range::new(max(r.start, ins.number + 1), r.end),
                        ),
                    ));
                }
                if r.start <= ins.number {
                    *ranges.get_part_mut(ins.property) =
                        Range::new(r.start, min(r.end, ins.number + 1))
                } else {
                    continue 'recur;
                }
            }
        }
        stack.push((&work_flow.otherwise, ranges))
    }
    total
}

pub fn solve_part_1(file: &str) -> Option<IntType> {
    let (hashmap, parts) = parse_file(file);
    Some(
        parts
            .iter()
            .map(|part| {
                let part_range = Ranges {
                    x: Range::new(part.x, part.x + 1),
                    m: Range::new(part.m, part.m + 1),
                    a: Range::new(part.a, part.a + 1),
                    s: Range::new(part.s, part.s + 1),
                };
                recur_stack(&hashmap, part_range) * part.get_total() as IntType
            })
            .sum(),
    )
}

pub fn solve_part_2(file: &str) -> Option<IntType> {
    const START: RangeInt = 1;
    const END: RangeInt = 4001; // range is exclusive

    let (hashmap, _) = parse_file(file);

    let ranges = Ranges {
        x: Range::new(START, END),
        m: Range::new(START, END),
        a: Range::new(START, END),
        s: Range::new(START, END),
    };

    Some(recur_stack(&hashmap, ranges))
}

const DAY: u8 = 19;

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
        assert_eq!(solve_part_1(&content), Some(19114));
        let content = inputs::get_file(DAY, InputType::Input);
        assert_eq!(solve_part_1(&content), Some(325952));
    }

    #[test]
    fn solves_second_problem() {
        let content = inputs::get_file(DAY, InputType::Sample);
        assert_eq!(solve_part_2(&content), Some(167409079868000));
        let content = inputs::get_file(DAY, InputType::Input);
        assert_eq!(solve_part_2(&content), Some(125744206494820));
    }
}
