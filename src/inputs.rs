use std::fs;

#[derive(Debug, PartialEq)]
pub enum InputType {
    Sample,
    Input,
}

impl InputType {
    fn folder_path(&self) -> &str {
        match self {
            InputType::Sample => "examples",
            InputType::Input => "inputs",
        }
    }
}

pub fn get_file(day: u8, input_type: InputType) -> String {
    let folder = input_type.folder_path();
    fs::read_to_string(format!("./data/{folder}/{day:02}.txt")).expect("File dose not exist")
}

pub fn get_file_part(day: u8, input_type: InputType, part: u8) -> String {
    let folder = input_type.folder_path();
    fs::read_to_string(format!("./data/{folder}/{day:02}-{part}.txt")).expect("File dose not exist")
}

fn _main() {
    println!("content: {:?}", get_file(2, InputType::Sample));
    println!("content: {:?}", get_file_part(1, InputType::Sample, 1));
    println!("content: {:?}", get_file_part(1, InputType::Sample, 2));
}

