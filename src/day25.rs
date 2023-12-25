use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;
use once_cell::sync::Lazy;
use rand::prelude::*;
use regex::Regex;

type IntType = usize;

const REGEX_START: &str = r"(\w{3}):";
const REGEX_LINKS: &str = r" (\w{3})";

fn parse(file: &str) -> (Vec<String>, Vec<Vec<usize>>) {
    static RE_START: Lazy<Regex> = Lazy::new(|| Regex::new(REGEX_START).unwrap());
    static RE_LINKS: Lazy<Regex> = Lazy::new(|| Regex::new(REGEX_LINKS).unwrap());

    let lines = file
        .lines()
        .map(|line| {
            let (_, [start]) = RE_START.captures(line).unwrap().extract();
            let links = RE_LINKS
                .captures_iter(line)
                .map(|c| c.extract())
                .map(|(_, [link])| link.to_string())
                .collect_vec();
            (start.to_string(), links)
        })
        .collect_vec();

    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    for (start, connections) in lines {
        let e = graph.entry(start.clone()).or_default();
        e.append(&mut connections.clone());
        e.dedup();
        for connection in connections {
            let e = graph.entry(connection).or_default();
            e.push(start.clone());
            e.dedup();
        }
    }

    let keys = graph.keys().cloned().collect_vec();
    let mut vec_graph: Vec<Vec<usize>> = vec![vec![]; keys.len()];

    for (key, value) in graph {
        let index = keys.iter().position(|r| *r == key).unwrap();

        for v in value {
            vec_graph[index].push(keys.iter().position(|r| *r == v).unwrap())
        }
    }

    (keys, vec_graph)
}

fn shortest_path(graph: &Vec<Vec<usize>>, start: usize, end: usize) -> Option<Vec<usize>> {
    let mut node_map = HashMap::new();
    node_map.insert(start, None);

    let mut seen = HashSet::new();
    seen.insert(start);

    let mut stack = VecDeque::new();
    stack.push_back(start);

    while let Some(current) = stack.pop_front() {
        if current == end {
            let mut path = vec![current];
            while let Some(prev) = node_map[path.last().unwrap()] {
                path.push(prev);
            }
            path.reverse();
            return Some(path);
        }
        for &next in graph[current].iter() {
            if !seen.contains(&next) {
                seen.insert(next);
                // parent stuff
                node_map.insert(next, Some(current));
                stack.push_back(next);
            }
        }
    }
    None
}

fn count_group_size(graph: &Vec<Vec<usize>>, start: usize) -> usize {
    let mut seen = HashSet::new();
    seen.insert(start);

    let mut stack = vec![start];
    while let Some(cur) = stack.pop() {
        for &next in graph[cur].iter() {
            if !seen.contains(&next) {
                seen.insert(next);
                stack.push(next);
            }
        }
    }
    seen.len()
}

pub fn solve_part_1(file: &str) -> Option<IntType> {
    let (_keys, graph) = parse(file);

    let mut rng = rand::thread_rng();

    loop {
        let [s, t] = (0..graph.len()).choose_multiple(&mut rng, 2)[..2] else {
            unreachable!()
        };

        let mut graph = graph.clone();
        for _ in 0..3 {
            // find shortest path
            let path = shortest_path(&graph, s, t).expect("graph has path");

            // remove edges from graph
            for (&s1, &t1) in path.iter().tuple_windows() {
                let index = graph[s1].iter().position(|x| *x == t1).unwrap();
                graph[s1].swap_remove(index);

                let index = graph[t1].iter().position(|x| *x == s1).unwrap();
                graph[t1].swap_remove(index);
            }
        }

        if shortest_path(&graph, s, t).is_none() {
            // found it
            let size = count_group_size(&graph, s);
            return Some(size * (graph.len() - size));
        }
    }
}

const DAY: u8 = 25;

#[allow(unused)]
pub fn main(file: &str) {
    println!("Solving Day {}", DAY);
    println!("  part 1: {:?}", solve_part_1(&file));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inputs::{self, InputType};

    #[test]
    fn solves_first_problem() {
        let content = inputs::get_file(DAY, InputType::Sample);
        assert_eq!(solve_part_1(&content), Some(54))
    }
}
