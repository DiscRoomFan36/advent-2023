mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

use std::{fmt::Display, time::Instant};

use inputs::{get_file, InputType};
mod helpers;
mod inputs;

fn main() {
    println!("Hello Advent");
    let start = Instant::now();

    // time(1, 1, day01::solve_part_1);
    // time(1, 2, day01::solve_part_2);
    // time(2, 1, day02::solve_part_1);
    // time(2, 2, day02::solve_part_2);
    // time(3, 1, day03::solve_part_1);
    // time(3, 2, day03::solve_part_2);
    // time(4, 1, day04::solve_part_1);
    // time(4, 2, day04::solve_part_2);
    // time(5, 1, day05::solve_part_1);
    // time(5, 2, day05::solve_part_2);
    // time(6, 1, day06::solve_part_1);
    // time(6, 2, day06::solve_part_2);
    // time(7, 1, day07::solve_part_1);
    // time(7, 2, day07::solve_part_2);
    // time(8, 1, day08::solve_part_1);
    // time(8, 2, day08::solve_part_2);
    // time(9, 1, day09::solve_part_1);
    // time(9, 2, day09::solve_part_2);
    // time(10, 1, day10::solve_part_1);
    // time(10, 2, day10::solve_part_2);
    // time(11, 1, day11::solve_part_1);
    // time(11, 2, day11::solve_part_2);
    // time(12, 1, day12::solve_part_1);
    // time(12, 2, day12::solve_part_2);
    // time(13, 1, day13::solve_part_1);
    // time(13, 2, day13::solve_part_2);
    // time(14, 1, day14::solve_part_1);
    // time(14, 2, day14::solve_part_2);
    // time(15, 1, day15::solve_part_1);
    // time(15, 2, day15::solve_part_2);
    // time(16, 1, day16::solve_part_1);
    // time(16, 2, day16::solve_part_2);
    // time(17, 1, day17::solve_part_1);
    // time(17, 2, day17::solve_part_2);
    // time(18, 1, day18::solve_part_1);
    // time(18, 2, day18::solve_part_2);
    // time(19, 1, day19::solve_part_1);
    // time(19, 2, day19::solve_part_2);
    // time(20, 1, day20::solve_part_1);
    // time(20, 2, day20::solve_part_2);
    // time(21, 1, day21::solve_part_1);
    // time(21, 2, day21::solve_part_2);
    // time(22, 1, day22::solve_part_1);
    // time(22, 2, day22::solve_part_2);
    time(23, 1, day23::solve_part_1);
    time(23, 2, day23::solve_part_2);
    // time(24, 1, day24::solve_part_1);
    // time(24, 2, day24::solve_part_2);
    // time(25, 1, day25::solve_part_1);

    let end = Instant::now();
    println!("Total Time : {:?}", end.duration_since(start));
}

fn time<T, F: Fn(&str) -> Option<T>>(day: u8, part: u8, f: F)
where
    T: Display + Default,
{
    let start = Instant::now();
    let result = f(&get_file(day, InputType::Input));
    let end = Instant::now();

    let result = result.unwrap_or_default();

    println!(
        "Day {day:>2} Part {part}: {result:<15} | Time: {:?}",
        end.duration_since(start)
    )
}
