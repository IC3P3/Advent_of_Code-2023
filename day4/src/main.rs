use regex::Regex;
use std::fs::read_to_string;
use std::time::Instant;
use std::vec;

type Scratchcard = (u64, Vec<u64>, Vec<u64>);
fn main() {
    let input = read_vector_from_file(&String::from("resources/input.txt"));

    let start = Instant::now();

    let combined_parts_numbers = get_combined_winning_number(&input);

    println!("Answer Part 1: {}", combined_parts_numbers);

    let duration = start.elapsed();
    println!("Time elapsed for day4 part1 is: {:?}", duration);

    let start = Instant::now();

    let card_count = get_total_card_count(&input);

    println!("Answer Part 2: {}", card_count);

    let duration = start.elapsed();
    println!("Time elapsed for day3 part2 is: {:?}", duration);
}

fn read_vector_from_file(filename: &String) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn extract_to_scratchcards(all_cards: &Vec<String>) -> Vec<Scratchcard> {
    let mut extracted_cards: Vec<Scratchcard> = vec![];
    let remove_card: Regex = Regex::new(r"\bCard\s*").unwrap();
    let remove_whitespaces = Regex::new(r"\s{1,}").unwrap();

    for scratchcard in all_cards {
        let splitted_string = scratchcard.split(":").collect::<Vec<&str>>();

        let game_id_string = remove_card.replace_all(&splitted_string[0], "");
        let game_id = game_id_string.parse().unwrap();

        let splitted_string = splitted_string[1].split("|").collect::<Vec<&str>>();

        let winning_numbers_string = remove_whitespaces
            .split(&splitted_string[0])
            .collect::<Vec<&str>>();

        let mut winning_numbers: Vec<u64> = vec![];
        for number in &winning_numbers_string[1..winning_numbers_string.len() - 1] {
            winning_numbers.push(number.parse().unwrap());
        }

        let numbers_string = remove_whitespaces
            .split(&splitted_string[1])
            .collect::<Vec<&str>>();

        let mut numbers: Vec<u64> = vec![];
        for number in &numbers_string[1..numbers_string.len()] {
            numbers.push(number.parse().unwrap());
        }

        extracted_cards.push((game_id, winning_numbers, numbers));
    }

    extracted_cards
}

// Part 1
fn get_combined_winning_number(lines: &Vec<String>) -> u64 {
    let all_games = extract_to_scratchcards(lines);
    let mut winnings = 0;

    for game in all_games {
        let won_games = game
            .2
            .iter()
            .filter(|&number| game.1.contains(number))
            .collect::<Vec<&u64>>();

        if won_games.len() != 0 {
            winnings += (2 as u64).pow(won_games.len() as u32 - 1);
        }
    }

    winnings
}

// Part 2
fn get_total_card_count(lines: &Vec<String>) -> usize {
    let all_games = extract_to_scratchcards(lines);
    let mut count = vec![1; all_games.len()];

    for i in 0..all_games.len() {
        let won_games = all_games[i]
            .2
            .iter()
            .filter(|&number| all_games[i].1.contains(number))
            .collect::<Vec<&u64>>();

        for j in 0..won_games.len() {
            count[i + j + 1] += count[i];
        }
    }

    count.iter().sum::<usize>()
}
