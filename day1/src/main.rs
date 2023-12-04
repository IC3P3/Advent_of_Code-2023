use std::fs::read_to_string;

fn main() {
    let _input = read_vector_from_file(&String::from("resources/input"));
}

fn read_vector_from_file(filename: &String) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
