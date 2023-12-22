use core::fmt;

use grid::Grid;
use once_cell::sync::Lazy;
use rayon::prelude::*;
use regex::Regex;

use crate::helpers::array_helpers::contains_only;

type IntType = usize;

#[derive(Debug, Clone, Copy)]
struct Brick {
    x1: usize,
    y1: usize,
    z1: usize,

    x2: usize,
    y2: usize,
    z2: usize,
}
impl fmt::Display for Brick {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{},{},{}~{},{},{}",
            self.x1, self.y1, self.z1, self.x2, self.y2, self.z2
        )
    }
}

const REGEX: &str = r"(\d+),(\d+),(\d+)~(\d+),(\d+),(\d+)";
fn parse(file: &str) -> Vec<Brick> {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(REGEX).unwrap());
    file.lines()
        .map(|line| {
            let (_, [x1, y1, z1, x2, y2, z2]) = RE.captures(line).unwrap().extract();
            Brick {
                x1: x1.parse().unwrap(),
                y1: y1.parse().unwrap(),
                z1: z1.parse().unwrap(),
                x2: x2.parse().unwrap(),
                y2: y2.parse().unwrap(),
                z2: z2.parse().unwrap(),
            }
        })
        .collect()
}

fn brick_collisions(
    layers: &Vec<Grid<Option<usize>>>,
    brick: Brick,
    move_down: usize,
) -> Option<Vec<usize>> {
    let mut collisions = vec![];
    for y in brick.y1..=brick.y2 {
        for x in brick.x1..=brick.x2 {
            if let Some(n) = layers[brick.z1 - move_down][(y, x)] {
                if !collisions.contains(&n) {
                    collisions.push(n);
                }
            }
        }
    }
    (!collisions.is_empty()).then_some(collisions)
}

fn place_brick(
    layers: &mut Vec<Grid<Option<usize>>>,
    brick: Brick,
    brick_i: usize,
    move_down: usize,
) {
    for z in (brick.z1 - move_down)..=(brick.z2 - move_down) {
        for y in brick.y1..=brick.y2 {
            for x in brick.x1..=brick.x2 {
                layers[z][(y, x)] = Some(brick_i);
            }
        }
    }
}

fn create_and_drop(
    mut bricks: Vec<Brick>,
) -> (Vec<(usize, Brick, usize)>, Vec<Grid<Option<usize>>>) {
    assert!(bricks
        .iter()
        .all(|b| b.x1 <= b.x2 && b.y1 <= b.y2 && b.z1 <= b.z2));

    let max_x = bricks.iter().max_by_key(|b| b.x2).unwrap().x2;
    let max_y = bricks.iter().max_by_key(|b| b.y2).unwrap().y2;
    let max_z = bricks.iter().max_by_key(|b| b.z2).unwrap().z2;

    let mut layers: Vec<Grid<Option<usize>>> = vec![];
    for _ in 0..max_z {
        layers.push(Grid::new(max_y + 1, max_x + 1))
    }

    bricks.sort_by_key(|b| b.z1);
    let bricks: Vec<(usize, &Brick)> = bricks.iter().enumerate().collect();

    // place the pieces
    let bricks = bricks
        .iter()
        .map(|(i, &brick)| {
            let mut downwards_move = 0;
            while brick.z1 - downwards_move > 0
                && brick_collisions(&layers, brick, downwards_move + 1).is_none()
            {
                downwards_move += 1;
            }
            // place brick
            place_brick(&mut layers, brick, *i, downwards_move);
            (*i, brick, downwards_move)
        })
        .collect();

    (bricks, layers)
}

pub fn solve_part_1(file: &str) -> Option<IntType> {
    let bricks = parse(file);
    let (bricks, layers) = create_and_drop(bricks);

    // count down many you can remove
    Some(
        bricks
            .par_iter()
            .filter(|(i, brick, move_down)| {
                let layer_above = (brick.z2 - move_down) + 1;
                let mut not_supporting = true;
                let mut no_brick_above = true;

                // loop over the layer, and check if a piece can move down, ignoring i
                // can just check above self
                'checking: for y in brick.y1..=brick.y2 {
                    for x in brick.x1..=brick.x2 {
                        if let Some(n) = layers[layer_above][(y, x)] {
                            no_brick_above = false;

                            let (_, checking_brick, move_down) = bricks[n];

                            let collisions =
                                brick_collisions(&layers, checking_brick, move_down + 1).unwrap();
                            if contains_only(&collisions, &[*i]) {
                                // there is another object supporting it
                                not_supporting = false;
                                break 'checking;
                            }
                        }
                    }
                }
                not_supporting || no_brick_above
            })
            .count(),
    )
}

pub fn solve_part_2(file: &str) -> Option<IntType> {
    let bricks = parse(file);
    let (bricks, layers) = create_and_drop(bricks);
    // when checking the above bricks, if it isn't supported, recur check
    Some(
        bricks
            .par_iter()
            .map(|(i, brick, move_down)| {
                let mut total_chain = 0;

                let mut layer_checking = brick.z2 - move_down + 1;
                let mut disappeared_bricks = vec![*i];
                while layer_checking < layers.len() && !disappeared_bricks.is_empty() {
                    let mut new_disappeared = vec![];
                    for b_i in disappeared_bricks.iter() {
                        let (_, b, _) = bricks[*b_i];
                        for y in b.y1..=b.y2 {
                            for x in b.x1..=b.x2 {
                                if let Some(n) = layers[layer_checking][(y, x)] {
                                    if new_disappeared.contains(&n)
                                        || disappeared_bricks.contains(&n)
                                    {
                                        continue;
                                    }
                                    let (_, checking_brick, checking_move_down) = bricks[n];
                                    let collisions = brick_collisions(
                                        &layers,
                                        checking_brick,
                                        checking_move_down + 1,
                                    )
                                    .unwrap();
                                    // contains only
                                    if contains_only(&collisions, &disappeared_bricks) {
                                        // new brick to drop
                                        new_disappeared.push(n);
                                        total_chain += 1;
                                    }
                                }
                            }
                        }
                    }
                    layer_checking += 1;
                    // filter those who wont matter
                    disappeared_bricks.retain(|b_i| {
                        let (_i, b, m_d) = bricks[*b_i];
                        layer_checking - 1 <= b.z2 - m_d
                    });

                    disappeared_bricks.append(&mut new_disappeared);
                }

                total_chain
            })
            .sum(),
    )
}

const DAY: u8 = 22;

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
        assert_eq!(solve_part_1(&content), Some(5))
    }

    #[test]
    fn solves_second_problem() {
        let content = inputs::get_file(DAY, InputType::Sample);
        assert_eq!(solve_part_2(&content), Some(7))
    }
}
