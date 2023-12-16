use std::cmp::max;

type IntType = u64;

type Position = [IntType; 2];

fn get_positions(file: &str) -> Vec<Position> {
    file.lines()
        .enumerate()
        .flat_map(|(j, line)| {
            line.bytes().enumerate().filter_map(move |(i, c)| {
                if c == b'#' {
                    Some([i as IntType, j as IntType])
                } else {
                    None
                }
            })
        })
        .collect()
}

fn empty_rows_and_cols(galaxies: &Vec<Position>) -> (Vec<IntType>, Vec<IntType>) {
    let (rows, cols) = galaxies.iter().fold((0, 0), |(rows, cols), [x, y]| {
        (max(rows, *x as usize), max(cols, *y as usize))
    });

    let mut rows = vec![false; rows + 1];
    let mut cols = vec![false; cols + 1];

    galaxies.iter().for_each(|galaxy| {
        rows[galaxy[0] as usize] = true;
        cols[galaxy[1] as usize] = true;
    });

    let rows = (0..rows.len())
        .filter(|&x| rows[x] == false)
        .map(|x| x as IntType)
        .collect();
    let cols = (0..cols.len())
        .filter(|&x| cols[x] == false)
        .map(|x| x as IntType)
        .collect();

    (rows, cols)
}

fn expand_galaxies(
    galaxies: &Vec<Position>,
    rows: &Vec<IntType>,
    cols: &Vec<IntType>,
    expansion: IntType,
) -> Vec<Position> {
    galaxies
        .iter()
        .map(|[x, y]| {
            let smaller_x = rows.iter().filter(|&&row| row < *x).count() as IntType;
            let smaller_y = cols.iter().filter(|&&col| col < *y).count() as IntType;
            [
                (x - smaller_x) + smaller_x * expansion,
                (y - smaller_y) + smaller_y * expansion,
            ]
        })
        .collect()
}

fn dist(pos1: Position, pos2: Position) -> IntType {
    pos1[0].abs_diff(pos2[0]) + pos1[1].abs_diff(pos2[1])
}

fn total_distances(galaxies: &Vec<Position>) -> IntType {
    (0..galaxies.len())
        .flat_map(|x| (x + 1..galaxies.len()).map(move |y| dist(galaxies[x], galaxies[y])))
        .sum()
}

pub fn solve_part_1(file: &str) -> Option<IntType> {
    let galaxies = get_positions(file);
    let (rows, cols) = empty_rows_and_cols(&galaxies);
    let galaxies = expand_galaxies(&galaxies, &rows, &cols, 2);

    Some(total_distances(&galaxies))
}

pub fn solve_part_2(file: &str) -> Option<IntType> {
    let galaxies = get_positions(file);
    let (rows, cols) = empty_rows_and_cols(&galaxies);
    let galaxies = expand_galaxies(&galaxies, &rows, &cols, 1_000_000);

    Some(total_distances(&galaxies))
}

const DAY: u8 = 11;

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
        assert_eq!(solve_part_1(&content), Some(374))
    }

    #[test]
    fn solves_second_problem() {
        let content = inputs::get_file(DAY, InputType::Sample);

        let galaxies = get_positions(&content);
        let (rows, cols) = empty_rows_and_cols(&galaxies);

        let galaxies1 = expand_galaxies(&galaxies, &rows, &cols, 10);
        assert_eq!(total_distances(&galaxies1), 1030);

        let galaxies2 = expand_galaxies(&galaxies, &rows, &cols, 100);
        assert_eq!(total_distances(&galaxies2), 8410);
    }
}
