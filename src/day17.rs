use crate::helpers::constructor::{file_to_grid, FromChar, Grid};
use crate::helpers::enums_and_types::{Direction, Position};
use std::collections::BinaryHeap;

type IntType = u32;

impl FromChar for u8 {
    fn from_char(c: char) -> Self {
        match c {
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            _ => panic!(),
        }
    }
}

type DirAndCount = (Direction, u8);

fn next_directions(
    (start_dir, start_count): DirAndCount,
    (min, max): (u8, u8),
) -> Vec<DirAndCount> {
    if start_count < min {
        vec![(start_dir, start_count + 1)]
    } else {
        [
            match start_dir {
                Direction::Up => {
                    vec![(Direction::Left, 1), (Direction::Right, 1)]
                }
                Direction::Down => {
                    vec![(Direction::Left, 1), (Direction::Right, 1)]
                }
                Direction::Left => {
                    vec![(Direction::Up, 1), (Direction::Down, 1)]
                }
                Direction::Right => {
                    vec![(Direction::Up, 1), (Direction::Down, 1)]
                }
            },
            if start_count >= max {
                vec![]
            } else {
                vec![(start_dir, start_count + 1)]
            },
        ]
        .concat()
    }
}

fn next_position<T>(grid: &Grid<T>, (j, i): Position, dir: Direction) -> Option<Position> {
    match dir {
        Direction::Up => {
            if j > 0 {
                Some((j - 1, i))
            } else {
                None
            }
        }
        Direction::Down => {
            if j < grid.rows() - 1 {
                Some((j + 1, i))
            } else {
                None
            }
        }
        Direction::Left => {
            if i > 0 {
                Some((j, i - 1))
            } else {
                None
            }
        }
        Direction::Right => {
            if i < grid.cols() - 1 {
                Some((j, i + 1))
            } else {
                None
            }
        }
    }
}
fn next_checks(loss_grid: &Grid<u8>, min_and_max: (u8, u8), check: NodeState) -> Vec<NodeState> {
    next_directions(check.dir_and_count, min_and_max)
        .iter()
        .map(|&next_dir| {
            next_position(loss_grid, check.position, next_dir.0).and_then(|pos| {
                Some(NodeState {
                    position: pos,
                    dir_and_count: next_dir,
                    heat_level: check.heat_level + loss_grid[pos] as IntType,
                })
            })
        })
        .filter_map(|x| x)
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct NodeState {
    position: Position,
    dir_and_count: DirAndCount,
    heat_level: IntType,
}
impl NodeState {
    // lower the heat, the better
    fn score(&self) -> IntType {
        // IntType::MAX - self.heat_level
        IntType::MAX - self.position.0 as IntType - self.position.1 as IntType - self.heat_level
    }
}
impl Ord for NodeState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score().cmp(&other.score())
    }
}
impl PartialOrd for NodeState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

fn path_find(file: &str, (min_dist, max_dist): (u8, u8)) -> IntType {
    let loss_grid: Grid<u8> = file_to_grid(file);

    let mut lowest_heat_on: Grid<Vec<DirAndCount>> = Grid::new(loss_grid.rows(), loss_grid.cols());

    let mut node_stack = BinaryHeap::new();
    node_stack.push(NodeState {
        position: (0, 0),
        dir_and_count: (Direction::Right, 0),
        heat_level: 0,
    });

    while let Some(state) = node_stack.pop() {
        if (state.position == (loss_grid.rows() - 1, loss_grid.cols() - 1))
            && (state.dir_and_count.1 >= min_dist)
        {
            return state.heat_level;
        }

        if lowest_heat_on[state.position].contains(&state.dir_and_count) {
            continue;
        } else {
            lowest_heat_on[state.position].push(state.dir_and_count);
        }

        next_checks(&loss_grid, (min_dist, max_dist), state)
            .iter()
            .for_each(|&state| node_stack.push(state));
    }
    panic!()
}

pub fn solve_part_1(file: &str) -> Option<IntType> {
    Some(path_find(file, (0, 3)))
}

pub fn solve_part_2(file: &str) -> Option<IntType> {
    Some(path_find(file, (4, 10)))
}

const DAY: u8 = 17;

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
        let content = inputs::get_file_part(DAY, InputType::Sample, 1);
        assert_eq!(solve_part_1(&content), Some(102));
    }

    #[test]
    fn solves_second_problem() {
        let content = inputs::get_file_part(DAY, InputType::Sample, 1);
        assert_eq!(solve_part_2(&content), Some(94));
        let content = inputs::get_file_part(DAY, InputType::Sample, 2);
        assert_eq!(solve_part_2(&content), Some(71));
    }
}
