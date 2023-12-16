use itertools::Itertools;
use std::collections::LinkedList;

type IntType = u32;

fn hash(str: &str) -> u8 {
    str.bytes()
        .fold(0, |z, u| z.wrapping_add(u).wrapping_mul(17))
}

pub fn solve_part_1(file: &str) -> Option<IntType> {
    Some(
        file.split(|c| c == ',')
            .map(|step| hash(step) as IntType)
            .sum(),
    )
}

pub fn solve_part_2(file: &str) -> Option<IntType> {
    let mut boxes: Vec<LinkedList<(&str, u8)>> = vec![LinkedList::new(); 256];

    file.split(|c| c == ',').for_each(|operation| {
        let (ident, action) = operation
            .split(|c| c == '=' || c == '-')
            .collect_tuple()
            .unwrap();

        let list = &mut boxes[hash(ident) as usize];

        match !action.is_empty() {
            true => {
                let focal_length = action.parse().unwrap();
                match list.iter_mut().find(|(id, _)| *id == ident) {
                    Some((_, focal)) => {
                        *focal = focal_length;
                    }
                    None => {
                        list.push_back((ident, focal_length));
                    }
                }
            }
            false => {
                let ele = list.iter().enumerate().find(|(_, (id, _))| *id == ident);
                if let Some((i, _)) = ele {
                    let mut split_list = list.split_off(i);
                    split_list.pop_front();
                    list.append(&mut split_list);
                }
            }
        }
    });

    Some(
        boxes
            .iter()
            .enumerate()
            .map(|(i, ll)| {
                ll.iter()
                    .enumerate()
                    .map(|(j, (_, focal))| {
                        (i as IntType + 1) * (j as IntType + 1) * *focal as IntType
                    })
                    .sum::<IntType>()
            })
            .sum(),
    )
}

const DAY: u8 = 15;

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
    fn test_hashing() {
        assert_eq!(hash("HASH"), 52);
        assert_eq!(hash("rn"), hash("cm"));
    }

    #[test]
    fn solves_first_problem() {
        let content = inputs::get_file(DAY, InputType::Sample);
        assert_eq!(solve_part_1(&content), Some(1320))
    }

    #[test]
    fn solves_second_problem() {
        let content = inputs::get_file(DAY, InputType::Sample);
        assert_eq!(solve_part_2(&content), Some(145))
    }
}
