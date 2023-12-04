use std::fs::read_to_string;

// General
fn main() {
    let input = read_vector_from_file(&String::from("resources/input"));
    let calibration_value = get_combined_calibration_value(&input);

    println!("Answer Part 1: {}", calibration_value);

    let calibration_value = get_combined_calibration_value_written_numbers(&input);

    println!("Answer Part 2: {}", calibration_value);
}

fn read_vector_from_file(filename: &String) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

// Part 1
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

// Part 2
#[derive(Debug)]
struct Index {
    _pos: usize,
    num: u64,
}

fn get_combined_calibration_value_written_numbers(input_list: &Vec<String>) -> u64 {
    let mut calibration_value = 0;

    for line in input_list {
        calibration_value += get_calibration_value_written_numbers(line);
    }

    calibration_value
}

fn get_calibration_value_written_numbers(line: &String) -> u64 {
    let possible_numbers: Vec<(String, u64)> = vec![
        (String::from("one"), 1),
        (String::from("two"), 2),
        (String::from("three"), 3),
        (String::from("four"), 4),
        (String::from("five"), 5),
        (String::from("six"), 6),
        (String::from("seven"), 7),
        (String::from("eight"), 8),
        (String::from("nine"), 9),
        (String::from("1"), 1),
        (String::from("2"), 2),
        (String::from("3"), 3),
        (String::from("4"), 4),
        (String::from("5"), 5),
        (String::from("6"), 6),
        (String::from("7"), 7),
        (String::from("8"), 8),
        (String::from("9"), 9),
    ];

    let found_numbers: Vec<Index> = possible_numbers
        .iter()
        .filter_map(|(word, num)| line.find(word).map(|_pos| Index { _pos, num: *num }))
        .collect();

    let temp = get_number(&found_numbers);

    println!("{}", temp);

    temp
}

fn get_number(numbers: &Vec<Index>) -> u64 {
    let mut first_number = &numbers[0];
    let mut last_number = &numbers[0];

    for number in numbers {
        if number._pos < first_number._pos {
            first_number = number;
        }

        if number._pos > last_number._pos {
            last_number = number;
        }
    }

    println!("{} {} | {:?}", first_number.num, last_number.num, numbers);

    (first_number.num.to_string() + &last_number.num.to_string())
        .parse()
        .unwrap()
}
