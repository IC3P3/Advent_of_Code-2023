use std::fs::read_to_string;

fn main() {
    let _input = read_vector_from_file(&String::from("resources/input"));

    println!("{:?}", _input);
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

fn get_calibration_value(input_list: &String) -> u64 {}
