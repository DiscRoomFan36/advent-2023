use std::{
    cmp::{max, min},
    iter::zip,
};

type IntType = u32;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    N,
    E,
    S,
    W,
}

type Grid<T> = Vec<Vec<T>>;
type Position = (usize, usize);

#[derive(Debug, Clone, Copy)]
enum PipeType {
    Dot,
    S,
    LineUp,
    LineCross,
    L,
    J,
    Seven,
    F,
}
impl PipeType {
    fn new(char: char) -> Self {
        match char {
            '.' => Self::Dot,
            'S' => Self::S,
            '|' => Self::LineUp,
            '-' => Self::LineCross,
            'L' => Self::L,
            'J' => Self::J,
            '7' => Self::Seven,
            'F' => Self::F,
            _ => panic!(),
        }
    }
    fn file_to_grid(file: &str) -> (Grid<PipeType>, Position) {
        let mut start_pos = (0, 0);
        let grid: Grid<PipeType> = file
            .lines()
            .enumerate()
            .map(|(j, line)| {
                line.chars()
                    .enumerate()
                    .map(|(i, char)| {
                        if char == 'S' {
                            start_pos = (i, j);
                        }
                        PipeType::new(char)
                    })
                    .collect()
            })
            .collect();

        (grid, start_pos)
    }
    fn has_exit(self, dir: Direction) -> bool {
        match self {
            PipeType::Dot => false,
            PipeType::S => false,
            PipeType::LineUp => dir == Direction::N || dir == Direction::S,
            PipeType::LineCross => dir == Direction::E || dir == Direction::W,
            PipeType::L => dir == Direction::N || dir == Direction::E,
            PipeType::J => dir == Direction::N || dir == Direction::W,
            PipeType::Seven => dir == Direction::S || dir == Direction::W,
            PipeType::F => dir == Direction::S || dir == Direction::E,
        }
    }
    fn next_pipe(
        grid: &Grid<PipeType>,
        (cur_x, cur_y): Position,
        (prev_x, prev_y): Position,
    ) -> Position {
        let cur_pipe = grid[cur_y][cur_x];
        match cur_pipe {
            pipe if pipe.has_exit(Direction::N) && cur_y <= prev_y => (cur_x, cur_y - 1),
            pipe if pipe.has_exit(Direction::S) && cur_y >= prev_y => (cur_x, cur_y + 1),
            pipe if pipe.has_exit(Direction::W) && cur_x <= prev_x => (cur_x - 1, cur_y),
            pipe if pipe.has_exit(Direction::E) && cur_x >= prev_x => (cur_x + 1, cur_y),
            _ => panic!(),
        }
    }
    fn find_connections(grid: &Grid<PipeType>, (x, y): Position) -> [Position; 2] {
        let check_directions = vec![Direction::W, Direction::E, Direction::N, Direction::S];
        let adjacencies = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

        let adjacencies: Vec<(i32, i32)> = adjacencies
            .iter()
            .map(|(i, j)| (i + x as i32, j + y as i32))
            .collect();

        let exits: Vec<Position> = zip(adjacencies, check_directions)
            .filter(|((x, y), dir)| {
                let mut answer = false;
                if let Some(y) = grid.get(*y as usize) {
                    if let Some(x) = y.get(*x as usize) {
                        answer = x.has_exit(*dir)
                    }
                }
                answer
            })
            .map(|((x, y), _)| (x as usize, y as usize))
            .collect();
        assert_eq!(exits.len(), 2);
        [exits[0], exits[1]]
    }
}

pub fn solve_part_1(file: &str) -> Option<IntType> {
    let (grid, start_pos) = PipeType::file_to_grid(file);

    let starting = PipeType::find_connections(&grid, start_pos);

    let mut traveler = (starting[0], start_pos);
    let mut count = 1;
    while traveler.0 != start_pos {
        let (cur, prev) = traveler;
        let next_pipe = PipeType::next_pipe(&grid, cur, prev);
        traveler.1 = cur;
        traveler.0 = next_pipe;
        count += 1;
    }
    Some(count / 2)
}

fn grid_to_flood_grid((x, y): Position) -> Position {
    (x * 2 + 1, y * 2 + 1)
}
fn midpoint_flooded(pos1: Position, pos2: Position) -> Position {
    let (x1, y1) = pos1;
    let (x2, y2) = pos2;
    (
        ((max(x1, x2) - min(x1, x2)) / 2 + min(x1, x2)),
        ((max(y1, y2) - min(y1, y2)) / 2 + min(y1, y2)),
    )
}
fn set_pipe_wall(flooder_grid: &mut Grid<bool>, pos: Position) {
    let (x, y) = grid_to_flood_grid(pos);
    flooder_grid[y][x] = true;
}
fn set_midpoint(flooder_grid: &mut Grid<bool>, pos1: Position, pos2: Position) {
    let (pos1, pos2) = (grid_to_flood_grid(pos1), grid_to_flood_grid(pos2));
    let (x, y) = midpoint_flooded(pos1, pos2);
    flooder_grid[y][x] = true;
}

fn flood_fill(flooder_grid: &mut Grid<bool>, pos: Position) {
    let (x, y) = pos;
    flooder_grid[y][x] = true;

    if x > 0 && (flooder_grid[y][x - 1] == false) {
        flood_fill(flooder_grid, (x - 1, y));
    }
    if x < flooder_grid[y].len() - 1 && (flooder_grid[y][x + 1] == false) {
        flood_fill(flooder_grid, (x + 1, y));
    }

    if y > 0 && (flooder_grid[y - 1][x] == false) {
        flood_fill(flooder_grid, (x, y - 1));
    }
    if y < flooder_grid.len() - 1 && (flooder_grid[y + 1][x] == false) {
        flood_fill(flooder_grid, (x, y + 1));
    }
}

pub fn solve_part_2(file: &str) -> Option<IntType> {
    let (grid, start_pos) = PipeType::file_to_grid(file);
    let mut flooder_grid: Grid<bool> = (0..grid.len() * 2 + 1)
        .map(|_| (0..grid[0].len() * 2 + 1).map(|_| false).collect())
        .collect();

    let starting = PipeType::find_connections(&grid, start_pos);
    let mut traveler = (starting[0], start_pos);

    set_pipe_wall(&mut flooder_grid, traveler.0);
    set_midpoint(&mut flooder_grid, traveler.0, traveler.1);

    // place walls,
    while traveler.0 != start_pos {
        let (cur, prev) = traveler;
        let next_pipe = PipeType::next_pipe(&grid, cur, prev);
        traveler.1 = cur;
        traveler.0 = next_pipe;

        set_pipe_wall(&mut flooder_grid, traveler.0);
        set_midpoint(&mut flooder_grid, traveler.0, traveler.1);
    }

    // flood fill from outside the map
    flood_fill(&mut flooder_grid, (0, 0));

    // count the number of un-flooded tiles, that align with the beginning grid
    let mut count = 0;
    for j in 0..grid.len() {
        for i in 0..grid[j].len() {
            let (x, y) = grid_to_flood_grid((i, j));
            if flooder_grid[y][x] == false {
                count += 1;
            }
        }
    }
    Some(count)
}

const DAY: u8 = 10;

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
    fn travels_alone_pipe() {
        let file = inputs::get_file_part(DAY, InputType::Sample, 1);
        let (grid, _) = PipeType::file_to_grid(&file);
        assert_eq!(PipeType::next_pipe(&grid, (2, 1), (1, 1)), (3, 1));
    }

    #[test]
    fn solves_first_problem() {
        let content = inputs::get_file_part(DAY, InputType::Sample, 1);
        assert_eq!(solve_part_1(&content), Some(4));
        let content = inputs::get_file_part(DAY, InputType::Sample, 2);
        assert_eq!(solve_part_1(&content), Some(4));
        let content = inputs::get_file_part(DAY, InputType::Sample, 3);
        assert_eq!(solve_part_1(&content), Some(8));
        let content = inputs::get_file_part(DAY, InputType::Sample, 4);
        assert_eq!(solve_part_1(&content), Some(8));
    }

    #[test]
    fn solves_second_problem() {
        let content = inputs::get_file_part(DAY, InputType::Sample, 5);
        assert_eq!(solve_part_2(&content), Some(4));
        let content = inputs::get_file_part(DAY, InputType::Sample, 6);
        assert_eq!(solve_part_2(&content), Some(4));
        let content = inputs::get_file_part(DAY, InputType::Sample, 7);
        assert_eq!(solve_part_2(&content), Some(8));
        let content = inputs::get_file_part(DAY, InputType::Sample, 8);
        assert_eq!(solve_part_2(&content), Some(10))
    }
}
