use regex::Regex;
use std::fs;
struct MulData {
    first_number: u128,
    second_number: u128,
}
fn find_and_extract_muldata_from_valid_substrings(input: &str) -> Vec<MulData> {
    let regex = Regex::new(r"mul\((\d+),\s*(\d+)\)").unwrap();
    regex
        .captures_iter(input)
        .map(|cap| MulData {
            first_number: cap[1].parse().unwrap(),
            second_number: cap[2].parse().unwrap(),
        })
        .collect()
}
fn main() {
    let text_string = fs::read_to_string("data.txt").unwrap();
    let text: &str = &text_string;
    let data = find_and_extract_muldata_from_valid_substrings(&text);
    let mut total_product: u128 = 0;
    for mul_data in data {
        total_product += mul_data.first_number * mul_data.second_number;
    }
    println!("{total_product}")
}
