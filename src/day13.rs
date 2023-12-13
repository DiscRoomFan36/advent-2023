use grid::Grid;

type IntType = u16;

fn differences<T: Eq>(grid: &Grid<T>, start: usize, end: usize) -> IntType {
    if (end - start) % 2 == 0 {
        return IntType::MAX;
    }
    let mut count = 0;
    for i in 0..=(end - start) / 2 {
        for j in 0..grid.cols() {
            if grid[(i + start, j)] != grid[(end - i, j)] {
                count += 1;
            }
        }
    }
    count
}

fn reflection_rows_smudged<T: Eq>(grid: &Grid<T>, num_differences: IntType) -> Option<usize> {
    for dist_check in 0..grid.rows() {
        if differences(grid, dist_check, grid.rows() - 1) == num_differences {
            return Some((grid.rows() - dist_check) / 2 + dist_check);
        }
        if differences(grid, 0, grid.rows() - 1 - dist_check) == num_differences {
            return Some((grid.rows() - dist_check) / 2);
        }
    }
    None
}

fn find_reflection_smudge<T: Eq>(grid: &mut Grid<T>, num_differences: IntType) -> IntType {
    if let Some(rows) = reflection_rows_smudged(&grid, num_differences) {
        return rows as IntType * 100;
    }
    // reflect to check cols
    grid.transpose();

    if let Some(cols) = reflection_rows_smudged(&grid, num_differences) {
        return cols as IntType;
    }

    panic!();
}

fn get_grids(file: &str) -> Vec<Grid<bool>> {
    file.lines()
        .map(|line| line.chars().map(|c| c == '#').collect::<Vec<bool>>())
        .collect::<Vec<Vec<bool>>>()
        .split(|line| line.len() == 0)
        .map(|grid| {
            grid.iter()
                .map(|vec| vec.clone())
                .collect::<Vec<Vec<bool>>>()
        })
        .map(|grid| {
            Grid::from_vec(
                grid.iter().map(|x| x.clone()).flatten().collect(),
                grid[0].len(),
            )
        })
        .collect()
}

pub fn solve_part_1(file: &str) -> Option<IntType> {
    let mut grids = get_grids(file);

    Some(
        grids
            .iter_mut()
            .map(|grid| find_reflection_smudge(grid, 0))
            .sum(),
    )
}

pub fn solve_part_2(file: &str) -> Option<IntType> {
    let mut grids = get_grids(file);

    Some(
        grids
            .iter_mut()
            .map(|grid| find_reflection_smudge(grid, 1))
            .sum(),
    )
}

const DAY: u8 = 13;

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
        assert_eq!(solve_part_1(&content), Some(405))
    }

    #[test]
    fn solves_second_problem() {
        let content = inputs::get_file(DAY, InputType::Sample);
        assert_eq!(solve_part_2(&content), Some(400))
    }
}
