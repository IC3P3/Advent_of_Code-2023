use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::Instant;
use std::vec;

// General
type Position = (usize, usize);
type Symbols = HashMap<Position, String>;

fn main() {
    let input = read_vector_from_file(&String::from("resources/input.txt"));

    let start = Instant::now();

    let combined_parts_numbers = get_combined_parts_numbers(&input);

    println!("Answer Part 1: {}", combined_parts_numbers);

    let duration = start.elapsed();
    println!("Time elapsed for day1 part1 is: {:?}", duration);
}

fn read_vector_from_file(filename: &String) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn add_to_map(pos: Position, value: &str, map: &mut Symbols) {
    for x in 0..value.len() {
        map.insert((pos.0 - x, pos.1), value.to_string());
    }
}

fn get_map_with_characters(data: &Vec<String>) -> Symbols {
    let mut symbols = Symbols::new();

    for (y, line) in data.iter().enumerate() {
        let mut number = String::new();

        for (x, character) in line.char_indices() {
            match character {
                '.' => {
                    if !number.is_empty() {
                        add_to_map((x - 1, y), &number, &mut symbols);
                        number.clear();
                    }
                }
                n if n.is_numeric() => {
                    number.push(n);

                    if x == 139 {
                        add_to_map((x, y), &number, &mut symbols);
                        number.clear();
                    }
                }
                _ => {
                    if !number.is_empty() {
                        add_to_map((x - 1, y), &number, &mut symbols);
                        number.clear();
                    }

                    symbols.insert((x, y), character.to_string());
                }
            }
        }
    }

    symbols
}

fn check_for_gears(pos: &Position, chars: &Symbols) -> Option<Vec<u64>> {
    assert!(pos.1 > 0);
    assert!(pos.1 < 139);
    let (index, row) = *pos;
    let mut result = vec![];

    for i in [0, 2] {
        if let Some(x) = chars.get(&(index + i - 1, row)) {
            if let Ok(num) = x.parse::<u64>() {
                result.push(num);
            }
        }
    }
    'row: for r in [0, 2] {
        for i in [1, 0, 2] {
            if let Some(x) = chars.get(&(index + i - 1, row + r - 1)) {
                if let Ok(num) = x.parse::<u64>() {
                    result.push(num);
                    if i == 1 {
                        // break, corners will be the same number, if any
                        continue 'row;
                    }
                }
            }
        }
    }

    if result.is_empty() {
        return None;
    }

    Some(result)
}

// Part 1
fn get_combined_parts_numbers(data: &Vec<String>) -> u64 {
    let characters: HashMap<(usize, usize), String> = get_map_with_characters(data);
    let mut part_numbers: u64 = 0;

    for (position, value) in characters.iter() {
        if value.parse::<u64>().is_ok() {
            continue;
        }

        if let Some(numbers) = check_for_gears(position, &characters) {
            part_numbers += numbers.iter().sum::<u64>();
        }
    }

    part_numbers
}
