use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn is_ascending_and_changing(vec: &[i32]) -> bool {
    vec.windows(2)
        .all(|w| w[0] <= w[1] && ((w[0] - w[1]).abs() <= 3) && ((w[0] - w[1]).abs() > 0))
}

fn is_descending_and_changing(vec: &[i32]) -> bool {
    vec.windows(2)
        .all(|w| w[0] >= w[1] && ((w[0] - w[1]).abs() <= 3) && ((w[0] - w[1]).abs() > 0))
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("data.txt")?;
    let reader = BufReader::new(file);

    let safe_reports = reader
        .lines()
        .map(|line_result| {
            let line = line_result.unwrap();
            let level: Vec<i32> = line
                .split_whitespace()
                .map(|level| level.parse::<i32>().unwrap())
                .collect();

            let is_safe: bool =
                is_ascending_and_changing(&level) || is_descending_and_changing(&level);

            is_safe
        })
        .filter(|&is_safe| is_safe)
        .count();

    println!("{safe_reports}");

    Ok(())
}
