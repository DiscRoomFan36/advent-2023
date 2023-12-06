use inputs::{get_file, InputType};
mod inputs;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

fn main() {
    println!("Hello Advent");

    day01::main(&get_file(1, InputType::Input));
    day02::main(&get_file(2, InputType::Input));
    day03::main(&get_file(3, InputType::Input));
    day04::main(&get_file(4, InputType::Input));
    day05::main(&get_file(5, InputType::Input));
    day06::main(&get_file(6, InputType::Input));
}
