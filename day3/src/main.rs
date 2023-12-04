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
    let char_positions = get_pos_of_all_characters(list_array);

    let mut combined_value = 0;

    for character_pos in char_positions {
        combined_value += get_gear_value(&character_pos, list_array);
    }

    combined_value
}

fn get_pos_of_all_characters(char_list: Vec<Vec<char>>) -> Vec<Position> {
    let mut characters_found: Vec<Position> = Vec::new();

    for (y, line) in char_list.iter().enumerate() {
        for (x, character) in line.iter().enumerate() {
            if character != &'.' {
                characters_found.push(Position {
                    x: x as i64,
                    y: y as i64,
                });
            }
        }
    }

    characters_found
}

fn get_gear_value(pos: &Position, array: Vec<Vec<char>>) -> u64 {
    let possible_positions: [Position; 8] = [
        Position { x: -1, y: -1 },
        Position { x: 0, y: -1 },
        Position { x: 1, y: -1 },
        Position { x: -1, y: 1 },
        Position { x: 0, y: 1 },
        Position { x: 1, y: 1 },
        Position { x: -1, y: 0 },
        Position { x: 1, y: 0 },
    ];

    let value = 0;

    value
}
