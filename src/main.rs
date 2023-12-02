use inputs::{load_file, InputType};

mod day01;
mod inputs;


fn main() {
    println!("Hello Advent");

	let day_1_data_sample = load_file(1, InputType::Sample(1));
	let day_1_data = load_file(1, InputType::Input);

    day01::main(&day_1_data_sample);
    day01::main(&day_1_data);
}
