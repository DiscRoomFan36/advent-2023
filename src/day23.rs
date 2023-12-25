use grid::Grid;
use itertools::Itertools;

use crate::helpers::constructor::{file_to_grid, FromChar};

type IntType = u16;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HikingTrailType {
    Path,
    Forest,
    SlopUp,
    SlopRight,
    SlopDown,
    SlopLeft,
}
impl FromChar for HikingTrailType {
    fn from_char(c: char) -> Self {
        match c {
            '.' => HikingTrailType::Path,
            '#' => HikingTrailType::Forest,
            '^' => HikingTrailType::SlopUp,
            '>' => HikingTrailType::SlopRight,
            'v' => HikingTrailType::SlopDown,
            '<' => HikingTrailType::SlopLeft,
            _ => panic!(),
        }
    }
}

use crate::helpers::enums_and_types::{Direction, DIRECTIONS};
use crate::helpers::positions_and_directions::next_position;

// fn trail_to_graph(
//     trail: Grid<HikingTrailType>,
//     one_way: bool,
// ) -> HashMap<Position, Vec<(usize, Position)>> {
//     // a graph that has a list of children and the length of the connection
//     let mut graph = HashMap::new();

// // direction down
// let start_pos = (0, 1);
// let end_pos = (trail.rows() - 1, trail.cols() - 2);

// // keep track of every junction
// let junction_list = vec![];

// let mut stack = vec![(start_pos, Direction::Down, 1, junction_list)];

// let mut longest_trail = 0;

// while let Some((pos, dir, steps, mut junction_list)) = stack.pop() {
//     let next_pos = next_position(pos, dir);
//     let next_step_count = steps + 1;
//     if junction_list.contains(&next_pos) {
//         continue;
//     }

//     if next_pos == end_pos {
//         if steps > longest_trail {
//             longest_trail = steps;
//         }
//         continue;
//     }

//     let next_directions = DIRECTIONS
//         .iter()
//         .filter(|&&d| d != dir.opposite())
//         .filter(|&&next_dir| {
//             let next_next_pos = next_position(next_pos, next_dir);

//             match trail[next_next_pos] {
//                 HikingTrailType::Forest => false,
//                 HikingTrailType::Path => true,
//                 _ if one_way == false => true,
//                 HikingTrailType::SlopUp => next_dir == Direction::Up,
//                 HikingTrailType::SlopRight => next_dir == Direction::Right,
//                 HikingTrailType::SlopDown => next_dir == Direction::Down,
//                 HikingTrailType::SlopLeft => next_dir == Direction::Left,
//             }
//         })
//         .collect_vec();

//     if next_directions.is_empty() {
//         continue;
//     } else if next_directions.len() == 1 {
//         // in a hallway
//         stack.push((
//             next_pos,
//             *next_directions[0],
//             next_step_count,
//             junction_list,
//         ))
//     } else {
//         // at a junction
//         junction_list.push(next_pos);
//         for &next_direction in next_directions {
//             stack.push((
//                 next_pos,
//                 next_direction,
//                 next_step_count,
//                 junction_list.clone(),
//             ))
//         }
//     }
// }

//     graph
// }

fn longest_trail(trail: Grid<HikingTrailType>, blocking_slopes: bool) -> IntType {
    // direction down
    let start_pos = (0, 1);
    let end_pos = (trail.rows() - 1, trail.cols() - 2);

    // keep track of every junction
    let junction_list = vec![];

    let mut stack = vec![(start_pos, Direction::Down, 1, junction_list)];

    let mut longest_trail = 0;

    while let Some((pos, dir, steps, mut junction_list)) = stack.pop() {
        let next_pos = next_position(pos, dir);
        let next_step_count = steps + 1;
        if junction_list.contains(&next_pos) {
            continue;
        }

        if next_pos == end_pos {
            if steps > longest_trail {
                longest_trail = steps;
            }
            continue;
        }

        let next_directions = DIRECTIONS
            .iter()
            .filter(|&&d| d != dir.opposite())
            .filter(|&&next_dir| {
                let next_next_pos = next_position(next_pos, next_dir);

                match trail[next_next_pos] {
                    HikingTrailType::Forest => false,
                    HikingTrailType::Path => true,
                    _ if blocking_slopes == false => true,
                    HikingTrailType::SlopUp => next_dir == Direction::Up,
                    HikingTrailType::SlopRight => next_dir == Direction::Right,
                    HikingTrailType::SlopDown => next_dir == Direction::Down,
                    HikingTrailType::SlopLeft => next_dir == Direction::Left,
                }
            })
            .collect_vec();

        if next_directions.is_empty() {
            continue;
        } else if next_directions.len() == 1 {
            // in a hallway
            stack.push((
                next_pos,
                *next_directions[0],
                next_step_count,
                junction_list,
            ))
        } else {
            // at a junction
            junction_list.push(next_pos);
            for &next_direction in next_directions {
                stack.push((
                    next_pos,
                    next_direction,
                    next_step_count,
                    junction_list.clone(),
                ))
            }
        }
    }

    longest_trail
}

pub fn solve_part_1(file: &str) -> Option<IntType> {
    let trail: Grid<HikingTrailType> = file_to_grid(file);
    Some(longest_trail(trail, true))
}

pub fn solve_part_2(file: &str) -> Option<IntType> {
    let trail: Grid<HikingTrailType> = file_to_grid(file);
    Some(longest_trail(trail, false))
}

const DAY: u8 = 23;

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
        assert_eq!(solve_part_1(&content), Some(94))
    }

    #[test]
    fn solves_second_problem() {
        let content = inputs::get_file(DAY, InputType::Sample);
        assert_eq!(solve_part_2(&content), Some(154))
    }
}
