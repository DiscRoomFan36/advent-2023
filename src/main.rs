use inputs::InputType;

use crate::inputs::{get_file_part, get_file};
mod inputs;

mod day01;
mod day02;

fn main() {
    println!("Hello Advent");

    day01::main(&get_file_part(1, InputType::Sample, 1));
    day01::main(&get_file(1, InputType::Input));

    day02::main(&get_file(2, InputType::Sample));
    day02::main(&get_file(2, InputType::Input));
}
