use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("data.txt")?;
    let reader = BufReader::new(file);

    let mut similarity_score = 0;

    let (list1, list2): (Vec<i32>, Vec<i32>) = reader
        .lines()
        .map(|line_result| {
            let line = line_result.unwrap();
            let words: Vec<&str> = line.split_whitespace().collect();
            (
                words[0].parse::<i32>().unwrap(),
                words[1].parse::<i32>().unwrap(),
            )
        })
        .unzip();

    for el in list1.iter() {
        let count = list2.iter().filter(|&x| x == el).count();
        similarity_score += (count as i32) * *el;
    }

    println!("{similarity_score}");

    Ok(())
}
