use std::{ops::Range, f32::consts::E};

use once_cell::sync::Lazy;
use regex::Regex;

const REGEX: &str = r"(\d+)";
fn line_to_digits(line: &str) -> Vec<u64> {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(REGEX).unwrap());
    RE.find_iter(line)
        .map(|m| m.as_str().trim_end().parse().unwrap_or(0))
        .collect()
}

// dest, src, range
// [ [1, 2, 3], [4, 5, 3] ]
fn map_a_to_b(map: &[Vec<u64>], number: u64) -> u64 {
    let range = map.iter().find(|m| (m[1]..m[1] + m[2]).contains(&number));

    match range {
        Some(vec) => (vec[1] as u32..number as u32).len() as u64 + vec[0],
        None => number,
    }
}

pub fn solve_part_1(file: &str) -> Option<u64> {
    let lines: Vec<Vec<u64>> = file.lines().map(|line| line_to_digits(line)).collect();

    let (seeds, lines) = (lines[0..1][0].clone(), &lines[3..]);
    let maps: Vec<Vec<Vec<u64>>> = lines
        .split(|line| line.is_empty())
        .map(|l| l.to_vec())
        .collect();
    assert!(maps
        .iter()
        .all(|map| map.iter().all(|range| range.len() == 3)));

    let maps: Vec<Vec<Vec<u64>>> = maps
        .iter()
        .filter(|m| !m.is_empty())
        .map(|m| m.to_vec())
        .collect();


    let seeds = maps
        .iter()
        .fold(seeds, |z, u| z.iter().map(|s| map_a_to_b(u, *s)).collect());

    Some(*seeds.iter().min().unwrap())
}


fn range_overlaps<T: std::cmp::PartialOrd>(r1:&Range<T>, r2:&Range<T>) -> bool {
    r1.start <= r2.start && r2.end <= r1.end
}

#[derive(Clone, Copy)]
struct Mapping {
    src: u64,
    dest: u64,
    dist: u64,
}
impl Mapping {
    fn new(src: u64, dest: u64, dist: u64) -> Self {
        Mapping { src, dest, dist }
    }
    
    fn to_range(self) -> SeedRange {
        self.src .. self.src+self.dist
    }

    // maps a single range, not more than one
    fn map_range(&self, range: &SeedRange) -> SeedRange {
        let (start, end) = (range.start, range.end);
        let (m_start, m_end) = (self.src, self.src + self.dist);

        assert!(range_overlaps(&self.to_range(), &range) || end <= m_start || m_end <= start);

        if end <= m_start || m_end <= start {
            return start..end;
        }
        
        start-self.src+self.dest .. end-self.src+self.dest
    }
}

struct Map{ mappings: Vec<Mapping> }

type SeedRange = Range<u64>;

// for a specitifc thing
type Seeds = Vec<SeedRange>;

impl Map {
    fn new_maps(lines: &[Vec<u64>]) -> Vec<Self> {
        let maps: Vec<Vec<Vec<u64>>> = lines
            .split(|line| line.is_empty())
            .map(|l| l.to_vec())
            .collect();
        assert!(maps
            .iter()
            .all(|map| map.iter().all(|range| range.len() == 3)));

        let maps: Vec<Vec<Vec<u64>>> = maps
            .iter()
            .filter(|m| !m.is_empty())
            .map(|m| m.to_vec())
            .collect();

        maps.iter().map(|map| {
            Map {
                mappings: map.iter().map(|mapping| {
                    Mapping { src: mapping[1], dest: mapping[0], dist: mapping[2] }
                }).collect()
            }
        }).collect()
        
    }

    // maps a single range, not more than one
    fn map_range(mapping: &Mapping, range: SeedRange) -> SeedRange{
        let (start, end) = (range.start, range.end);
        let (m_start, m_end) = (mapping.src, mapping.src + mapping.dist);

        assert!(range_overlaps(&mapping.to_range(), &range) || end <= m_start || m_end <= start);

        if end <= m_start || m_end <= start {
            return range;
        }
        
        start-mapping.src+mapping.dest .. end-mapping.src+mapping.dest
    }

    fn map_seeds_over_self(&self, seeds: SeedRange) -> SeedRange{
        for mapping in &self.mappings {
            if range_overlaps(&mapping.to_range(), &seeds) {
                return mapping.map_range(&seeds);
            }
        }
        seeds
    }

    fn shatter_map(mapping: &Mapping, range: &SeedRange) -> Vec<SeedRange> {
        let (start, end) = (range.start, range.end);
        let (m_start, m_end) = (mapping.src, mapping.src + mapping.dist);
        if m_start <= start && end <= m_end {
            return vec![start..end];
        } else if m_end <= start {
            return vec![start..end];
        } else if end <= m_start {
            return vec![start..end];
        } else if start < m_start {
            return [vec![start..m_start], Map::shatter_map(mapping, &(m_start..end))].concat();
        } else if m_end < end {
            return [Map::shatter_map(mapping, &(start..m_end)), vec![m_end..end]].concat();
        } else {
            panic!()
        }
    }

    fn shatter_and_map_over_self(&self, range: &SeedRange) -> Vec<SeedRange> {
        let (start, end) = (range.start, range.end);

        let shattered: Vec<Range<u64>> = self.mappings.iter().fold(vec![start..end], |z, mapping| {
            z.iter().flat_map(|r| {
                Map::shatter_map(mapping, r)
            }).collect()
        });

        shattered.iter().map(|range| self.map_seeds_over_self(range.clone())).collect()
    }

    fn transform_seeds(&self, ranges: &Seeds) -> Seeds {
        ranges.iter().flat_map(|range| self.shatter_and_map_over_self(range)).collect()
    }
}

pub fn solve_part_2(file: &str) -> Option<u64> {
    let lines: Vec<Vec<u64>> = file.lines().map(|line| line_to_digits(line)).collect();

    let (seeds, lines) = (lines[0..1][0].clone(), &lines[3..]);
    let seeds: Seeds = seeds.chunks_exact(2).map(|chunk| (chunk[0]..chunk[0]+chunk[1])).collect();
    let maps = Map::new_maps(lines);

    let seeds = maps.iter().fold(seeds, |z, u| {
        u.transform_seeds(&z)
    });

    Some(seeds.iter().min_by(|x, y| x.start.cmp(&y.start)).unwrap().start)
}

const DAY: u8 = 5;

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
    fn shatter_map_test() {
        let mapping = Mapping{ src: 3, dest: 9, dist: 2 };

        assert_eq!(Map::shatter_map(&mapping, &(1..3)), vec![1..3]);
        assert_eq!(Map::shatter_map(&mapping, &(1..4)), vec![1..3, 3..4]);
        assert_eq!(Map::shatter_map(&mapping, &(3..5)), vec![3..5]);
        assert_eq!(Map::shatter_map(&mapping, &(4..6)), vec![4..5, 5..6]);
        assert_eq!(Map::shatter_map(&mapping, &(5..7)), vec![5..7]);
        assert_eq!(Map::shatter_map(&mapping, &(2..6)), vec![(2..3), (3..5), (5..6)]);
    }

    #[test]
    fn solves_first_problem() {
        let content = inputs::get_file(DAY, InputType::Sample);
        assert_eq!(solve_part_1(&content), Some(35))
    }

    #[test]
    fn solves_second_problem() {
        let content = inputs::get_file(DAY, InputType::Sample);
        assert_eq!(solve_part_2(&content), Some(46))
    }
}
