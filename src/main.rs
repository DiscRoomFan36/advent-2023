use inputs::{InputType, get_file};
mod inputs;

mod day01;
mod day02;
mod day03;

fn main() {
    println!("Hello Advent");

    day01::main(&get_file(1, InputType::Input));

    day02::main(&get_file(2, InputType::Input));

    day03::main(&get_file(3, InputType::Input));
}
