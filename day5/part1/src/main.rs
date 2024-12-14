use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn check_if_page_is_correct(page: &[i32], rules: &HashMap<i32, Vec<i32>>) -> bool {
    for (i, nr) in page.iter().enumerate() {
        if let Some(dependencies) = rules.get(nr) {
            for &dependency in dependencies {
                if page[i + 1..].contains(&dependency) {
                    return false;
                }
            }
        }
    }
    true
}

fn main() {
    let file = File::open("data.txt").unwrap();
    let reader = BufReader::new(file);

    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut is_reading_rules = true;

    let mut middle_page_sum = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            is_reading_rules = !is_reading_rules;
            continue;
        };

        if is_reading_rules {
            let parts: Vec<i32> = line.split('|').map(|el| el.parse().unwrap()).collect();
            let (x, y) = (parts[0], parts[1]);

            rules.entry(y).or_insert_with(Vec::new).push(x);
        } else {
            let page: Vec<i32> = line
                .split(',')
                .map(|el| el.parse::<i32>().unwrap())
                .collect();

            if check_if_page_is_correct(&page, &rules) {
                middle_page_sum += page[page.len() / 2];
            }
        }
    }

    println!("{middle_page_sum}");
}
