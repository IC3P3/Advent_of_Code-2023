use std::fs::read_to_string;

// General
fn main() {
    let input = read_vector_from_file(&String::from("resources/input.txt"));
    let combined_parts_numbers = get_combined_parts_numbers(&input);

    println!("Answer Part 1: {}", combined_parts_numbers)
}

fn read_vector_from_file(filename: &String) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn vec_to_array(list: &Vec<String>) -> Vec<Vec<char>> {
    list.iter().map(|line| line.chars().collect()).collect()
}

// Part 1
struct Position {
    x: i64,
    y: i64,
}

fn get_combined_parts_numbers(list: &Vec<String>) -> u64 {
    let list_array = vec_to_array(list);
    let char_positions = get_pos_of_all_numbers(list_array.clone());

    get_gear_value(&char_positions, &list_array.clone())
}

fn get_pos_of_all_numbers(char_list: Vec<Vec<char>>) -> Vec<Position> {
    let mut characters_found: Vec<Position> = Vec::new();

    for (y, line) in char_list.iter().enumerate() {
        for (x, character) in line.iter().enumerate() {
            if character.is_numeric() {
                characters_found.push(Position {
                    x: x as i64,
                    y: y as i64,
                });
            }
        }
    }

    characters_found
}

fn get_gear_value(positions: &Vec<Position>, array: &Vec<Vec<char>>) -> u64 {
    let mut value = 0;
    let string_value: String;

    for (i, position) in positions.iter().enumerate() {
        if i == 0 {}

        if (position.x != positions[(i - 1) as usize].x - 1)
            && (position.y != positions[(i - 1) as usize].y)
        {}
    }

    value
}

fn is_gear_near_number(pos: &Position, array: &Vec<Vec<char>>, value: u64) -> bool {
    true
}
