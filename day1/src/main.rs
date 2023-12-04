use std::fs::read_to_string;

fn main() {
    let _input = read_vector_from_file(&String::from("resources/input"));
    let calibration_value = get_combined_calibration_value(&_input);

    println!("{}", calibration_value);
}

fn read_vector_from_file(filename: &String) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn get_combined_calibration_value(input_list: &Vec<String>) -> u64 {
    let mut calibration_value = 0;

    for line in input_list {
        calibration_value += get_calibration_value(line);
    }

    calibration_value
}

fn get_calibration_value(line: &String) -> u64 {
    let mut first_number: char = '0';
    let mut last_number: char = '0';

    for character in line.chars() {
        if character.is_numeric() && first_number == '0' {
            first_number = character;
        }

        if character.is_numeric() {
            last_number = character;
        }
    }

    (first_number.to_string() + &last_number.to_string())
        .parse()
        .unwrap()
}
