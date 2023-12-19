use inputs::{get_file, InputType};
mod helpers;
mod inputs;

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

fn main() {
    println!("Hello Advent");

    day01::main(&get_file(1, InputType::Input));
    day02::main(&get_file(2, InputType::Input));
    day03::main(&get_file(3, InputType::Input));
    day04::main(&get_file(4, InputType::Input));
    day05::main(&get_file(5, InputType::Input));
    day06::main(&get_file(6, InputType::Input));
    day07::main(&get_file(7, InputType::Input));
    day08::main(&get_file(8, InputType::Input));
    day09::main(&get_file(9, InputType::Input));
    day10::main(&get_file(10, InputType::Input));
    day11::main(&get_file(11, InputType::Input));
    day12::main(&get_file(12, InputType::Input));
    day13::main(&get_file(13, InputType::Input));
    day14::main(&get_file(14, InputType::Input));
    day15::main(&get_file(15, InputType::Input));
    day16::main(&get_file(16, InputType::Input));
    day17::main(&get_file(17, InputType::Input));
    day18::main(&get_file(18, InputType::Input));
}
