use std::collections::HashMap;

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
struct NodeIdent {
    ident: [u8; 3],
}
impl NodeIdent {
    fn from_slice(slice: &[u8]) -> Self {
        let mut ident: [u8; 3] = [0; 3];
        ident.copy_from_slice(&slice[..]);
        Self { ident }
    }
    fn create_node_map(file: &str) -> (Vec<u8>, HashMap<Self, (Self, Self)>) {
        let file: Vec<&str> = file.lines().collect();
        let instructions = file[0].as_bytes();
        let file = &file[2..];

        let mut hashmap: HashMap<Self, (Self, Self)> = HashMap::new();
        file.iter().for_each(|line| {
            let line = line.as_bytes();
            let ident = Self::from_slice(&line[0..3]);
            let left = Self::from_slice(&line[7..10]);
            let right = Self::from_slice(&line[12..15]);
            hashmap.insert(ident, (left, right));
        });

        (instructions.to_vec(), hashmap)
    }
    fn at_exit(&self, full_exit: bool) -> bool {
        match full_exit {
            true => self.ident == *b"ZZZ",
            false => {
                self.ident[2] == b'Z'
            }
        }
    }
    fn next_node(node: &Self, node_map: &HashMap<Self, (Self, Self)>, turn: u8) -> Self {
        let (left, right) = node_map.get(&node).unwrap();
        match turn {
            b'L' => *left,
            b'R' => *right,
            _ => panic!(),
        }
    }
}

fn dist_to_next_exit(node: &NodeIdent, node_map: &HashMap<NodeIdent, (NodeIdent, NodeIdent)>, instructions: &[u8], full_exit: bool) -> u64 {
    let mut cur = *node;
    let mut count = 0;
    while !cur.at_exit(full_exit) {
        let turn = instructions[count % instructions.len()];
        cur = NodeIdent::next_node(&cur, &node_map, turn);
        count += 1;
    }
    count as u64
}

// took from some guy on the AOC reddit, also the rust overflow checker didn't trigger for some reason, had to use u64
const fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}
const fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

pub fn solve_part_1(file: &str) -> Option<u64> {
    let (instructions, node_map) = NodeIdent::create_node_map(file);
    Some(dist_to_next_exit(&NodeIdent { ident: *b"AAA" }, &node_map, &instructions, true))
}

pub fn solve_part_2(file: &str) -> Option<u64> {
    let (instructions, node_map) = NodeIdent::create_node_map(file);

    let current_nodes: Vec<NodeIdent> = node_map.keys().filter(|key| {
        key.ident[2] == b'A'
    }).map(|n| *n).collect();
    
    let lcm = current_nodes.iter().fold(1, |z, node| {
        let dist = dist_to_next_exit(node, &node_map, &instructions, false);
        lcm(z, dist as usize)
    });

    Some(lcm as u64)
}

const DAY: u8 = 8;

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
        assert_eq!(solve_part_1(&content), Some(2));
        let content = inputs::get_file_part(DAY, InputType::Sample, 2);
        assert_eq!(solve_part_1(&content), Some(6));
    }

    #[test]
    fn solves_second_problem() {
        let content = inputs::get_file_part(DAY, InputType::Sample, 3);
        assert_eq!(solve_part_2(&content), Some(6));
    }
}