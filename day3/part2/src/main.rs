use regex::Regex;
use std::fs;

fn find_and_extract_muldata_from_valid_substrings(input: &str) -> Vec<String> {
    let combined_regex = Regex::new(r"mul\((\d+),\s*(\d+)\)|do\(\)|don't\(\)").unwrap();

    combined_regex
        .captures_iter(input)
        .map(|capture| capture.get(0).unwrap().as_str().to_string())
        .collect()
}

fn main() {
    let file_content = fs::read_to_string("data.txt").unwrap();
    let text: &str = &file_content;

    let operations = find_and_extract_muldata_from_valid_substrings(&text);

    let mut total_product: u128 = 0;
    let mut multiplication_enabled = true;

    for operation in operations {
        match operation.as_str() {
            _ if operation.starts_with("mul") && multiplication_enabled => {
                let regex = Regex::new(r"mul\((\d+),\s*(\d+)\)").unwrap();
                if let Some(capture) = regex.captures(&operation) {
                    let first_number: u128 = capture[1].parse().unwrap();
                    let second_number: u128 = capture[2].parse().unwrap();
                    total_product += first_number * second_number;
                }
            }
            "don't()" => {
                multiplication_enabled = false;
            }
            "do()" => {
                multiplication_enabled = true;
            }
            _ => {}
        }
    }

    println!("{total_product}")
}
