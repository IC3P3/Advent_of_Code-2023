use regex::Regex;
use std::fs::read_to_string;
use std::time::Instant;

// General
fn main() {
    let input = read_vector_from_file(&String::from("resources/input.txt"));

    let start = Instant::now();

    let get_possible_games = get_id_calculation(&input);

    println!("Answer Part 1: {}", get_possible_games);

    let duration = start.elapsed();
    println!("Time elapsed for day2 part1 is: {:?}", duration);

    let start = Instant::now();

    let get_calculated_power = get_power_of_games(&input);

    println!("Answer Part 2: {}", get_calculated_power);

    let duration = start.elapsed();
    println!("Time elapsed for day2 part2 is: {:?}", duration);
}

fn read_vector_from_file(filename: &String) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

// Part 1
fn get_id_calculation(list: &Vec<String>) -> u64 {
    let mut calculated_ids: u64 = 0;

    for line in list {
        let current_id = get_id_from_line(line);

        if is_possible(&(current_id.1.to_string())) {
            calculated_ids += current_id.0
        }
    }

    calculated_ids
}

fn get_id_from_line(line: &String) -> (u64, &str) {
    let split: Vec<&str> = line.split(':').collect::<Vec<&str>>();

    let id: u64 = split[0].replace("Game ", "").parse().unwrap();

    (id, split[1])
}

fn is_possible(line: &String) -> bool {
    let split_by_game = line.split(";").collect::<Vec<&str>>();

    for game in split_by_game {
        let split_by_color = game.split(",").collect::<Vec<&str>>();

        for color in split_by_color {
            if color.contains("red") {
                let red: u64 = color.replace("red", "").replace(" ", "").parse().unwrap();

                if red > 12 {
                    return false;
                }
            }

            if color.contains("green") {
                let green: u64 = color.replace("green", "").replace(" ", "").parse().unwrap();

                if green > 13 {
                    return false;
                }
            }

            if color.contains("blue") {
                let blue: u64 = color.replace("blue", "").replace(" ", "").parse().unwrap();

                if blue > 14 {
                    return false;
                }
            }
        }
    }

    true
}

// Part 2
fn get_power_of_games(list: &Vec<String>) -> u64 {
    let game_pattern: Regex = Regex::new(r"Game \d+:").unwrap();
    let mut power: u64 = 0;

    for line in list {
        let line_trimmed = game_pattern.replace_all(&line, "");
        let split: Vec<&str> = line_trimmed.split([';', ',']).collect::<Vec<&str>>();

        let color_split = sort_by_max_color(split);

        power += color_split.0 * color_split.1 * color_split.2;
    }

    power
}

fn sort_by_max_color(line: Vec<&str>) -> (u64, u64, u64) {
    let mut red: u64 = 0;
    let mut green: u64 = 0;
    let mut blue: u64 = 0;

    for color in line {
        match color {
            s if s.contains("red") => {
                let trim_red: u64 = s.replace("red", "").replace(" ", "").parse().unwrap();
                if trim_red > red {
                    red = trim_red;
                }
            }
            s if s.contains("green") => {
                let trim_green: u64 = s.replace("green", "").replace(" ", "").parse().unwrap();
                if trim_green > green {
                    green = trim_green;
                }
            }
            s if s.contains("blue") => {
                let trim_blue: u64 = s.replace("blue", "").replace(" ", "").parse().unwrap();
                if trim_blue > blue {
                    blue = trim_blue;
                }
            }
            _ => {}
        }
    }

    (red, green, blue)
}
