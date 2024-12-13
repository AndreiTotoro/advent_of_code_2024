use std::{
    error::Error,
    fs::File,
    i32,
    io::{BufRead, BufReader},
};

fn is_safe_after_removal(v: &Vec<i32>) -> bool {
    for i in 0..v.len() {
        let mut modified_v = v.clone();
        modified_v.remove(i);

        if is_ascending(&modified_v) || is_descending(&modified_v) {
            return true;
        }
    }
    false
}

fn is_ascending(v: &Vec<i32>) -> bool {
    v.windows(2)
        .all(|w| w[0] <= w[1] && (w[0] - w[1]).abs() > 0 && (w[0] - w[1]).abs() <= 3)
}

fn is_descending(v: &Vec<i32>) -> bool {
    v.windows(2)
        .all(|w| w[0] >= w[1] && (w[0] - w[1]).abs() > 0 && (w[0] - w[1]).abs() <= 3)
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("data.txt")?;
    let reader = BufReader::new(file);

    let safe_level_count = reader
        .lines()
        .filter(|line_result| {
            let line = line_result.as_ref().unwrap();
            let level: Vec<i32> = line
                .split_whitespace()
                .map(|el| el.parse::<i32>().unwrap())
                .collect();

            is_ascending(&level) || is_descending(&level) || is_safe_after_removal(&level)
        })
        .count();

    println!("{safe_level_count}");

    Ok(())
}
