use std::fs;

#[derive(Debug, PartialEq)]
pub enum InputType {
    Sample(u8),
    Input,
    None,
}

impl InputType {
    fn get_name(&self) -> &str {
        match self {
            Self::Sample(_) => "sample",
            Self::Input => "input",
            Self::None => "none",
        }
    }
}

pub fn get_input_path(day: u8, input_type: InputType) -> Option<String> {
    assert_ne!(input_type, InputType::None);

    let paths = fs::read_dir("./src/inputs").expect("Directory exists");

    for path in paths {
        let name = path.unwrap().file_name().into_string().unwrap();

        let (head, tail) = name.split_once('.').expect("File has a type");
        if tail != "txt" {
            continue;
        }

        let mut parts = head.split("-");

        let file_name = parts.next().expect("First part of file exists");
        if file_name != input_type.get_name() {
            continue;
        }

        if let Some(Ok(day_number)) = parts.next().map(|x| x.parse::<u8>()) {
            if day != day_number {
                continue;
            }
        } else {
            continue;
        }

        match input_type {
            InputType::Sample(num) => match num {
                1 => {
                    if parts.next() != None {
                        continue;
                    }
                    return Some(name);
                }
                _ => {
                    if let Some(Ok(in_num)) = parts.next().map(|x| x.parse::<u8>()) {
                        if in_num != num {
                            continue;
                        }
                        return Some(name);
                    }
                }
            },
            InputType::Input => {
                if parts.next() != None {
                    continue;
                }
                return Some(name);
            }
            InputType::None => panic!(),
        }
    }
    None
}

pub fn file_name_to_string_array(file_name: &str) -> Vec<String> {
    let contents =
        fs::read_to_string(format!("./src/inputs/{file_name}")).expect("Unable to read the file");
    let mut array: Vec<String> = Vec::new();
    for line in contents.lines() {
        array.push(line.to_string());
    }
    array
}

pub fn load_file(day: u8, input_type: InputType) -> Vec<String> {
    let name = get_input_path(day, input_type).expect("File was not there");
    file_name_to_string_array(&name)

}

fn _main() {
	let content = load_file(1, InputType::Sample(1));
	println!("content: {content:?}");
}
