use std::fs::read_to_string;

// General
fn main() {
    let input = read_vector_from_file(&String::from("resources/input"));
    let get_possible_games = get_id_calculation(&input);

    println!("Answer Part 1: {}", get_possible_games);
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
            println!("{}", color);

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
