use crate::helpers::constructor::line_to_digits;

type IntType = i64;

type HailType = f64;

#[derive(Debug, Clone, Copy)]
struct Hail {
    px: HailType,
    py: HailType,
    pz: HailType,
    vx: HailType,
    vy: HailType,
    vz: HailType,
}

const ZERO: HailType = 0.0;

impl Hail {
    fn to_pos(&self) -> (HailType, HailType, HailType) {
        (self.px, self.py, self.pz)
    }
    fn next_hail(&self) -> (HailType, HailType, HailType) {
        (self.px + self.vx, self.py + self.vy, self.pz + self.vz)
    }

    fn is_in_future(&self, x: HailType, y: HailType) -> bool {
        !((self.vx > ZERO && self.px > x)
            || (self.vx < ZERO && self.px < x)
            || (self.vy > ZERO && self.py > y)
            || (self.vy < ZERO && self.py < y))
    }

    fn intersection_between(&self, other: &Self, min: HailType, max: HailType) -> bool {
        let (x1, y1, ..) = self.to_pos();
        let (x2, y2, ..) = self.next_hail();
        let (x3, y3, ..) = other.to_pos();
        let (x4, y4, ..) = other.next_hail();

        let m1 = (y2 - y1) / (x2 - x1);
        let m2 = (y4 - y3) / (x4 - x3);

        let b1 = y1 - m1 * x1;
        let b2 = y3 - m2 * x3;

        // let (m1_p1, m1_p2) = (y2 - y1, x2 - x1);
        // let (m2_p1, m2_p2) = (y4 - y3, x4 - x3);

        // let (b1_p1, b1_p2) = ((y1 * m1_p2) - (m1_p1 * x1), m1_p2);
        // let (b2_p1, b2_p2) = ((y3 * m2_p2) - (m2_p1 * x3), m2_p2);

        if m1 == m2 {
            // if m1_p1 * m2_p2 == m2_p1 * m1_p2 {
            // parallel
            return false;
        }

        let x = (b2 - b1) / (m1 - m2);
        let y = (m1 * x) + b1;

        if !self.is_in_future(x, y) || !other.is_in_future(x, y) {
            false
        } else {
            // check intersections
            x >= min && x <= max && y >= min && y <= max
        }
    }
}

fn parse(file: &str) -> Vec<Hail> {
    file.lines()
        .map(|line| {
            let p = line_to_digits(line);
            assert_eq!(p.len(), 6);
            Hail {
                px: p[0],
                py: p[1],
                pz: p[2],
                vx: p[3],
                vy: p[4],
                vz: p[5],
            }
        })
        .collect()
}

fn count_intersections_2d(hail_stones: &Vec<Hail>, min: HailType, max: HailType) -> IntType {
    let mut intersections = 0;
    for i in 0..hail_stones.len() {
        for j in i + 1..hail_stones.len() {
            let hail_a = hail_stones[i];
            let hail_b = hail_stones[j];

            if hail_a.intersection_between(&hail_b, min, max) {
                intersections += 1;
            }
        }
    }

    intersections
}

pub fn solve_part_1(file: &str) -> Option<IntType> {
    let hail_stones = parse(file);
    const MIN: HailType = 200000000000000.0;
    const MAX: HailType = 400000000000000.0;
    Some(count_intersections_2d(&hail_stones, MIN, MAX))
}

pub fn solve_part_2(_file: &str) -> Option<IntType> {
    None
}

const DAY: u8 = 24;

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
        let hail_stones = parse(&content);
        assert_eq!(count_intersections_2d(&hail_stones, 7.0, 27.0), 2)
    }

    #[test]
    #[ignore = "not done, need to use z3"]
    fn solves_second_problem() {
        let content = inputs::get_file(DAY, InputType::Sample);
        assert_eq!(solve_part_2(&content), Some(47))
    }
}
