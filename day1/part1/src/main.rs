use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();
    let mut distance = 0;

    let file_result = File::open("data.txt");
    let file = match file_result {
        Err(e) => {
            println!("{e}");
            return;
        }
        Ok(file) => file,
    };

    let reader = BufReader::new(file);

    for line_result in reader.lines() {
        let line = match line_result {
            Ok(line) => line,
            Err(e) => {
                println!("{e}");
                return;
            }
        };
        let words: Vec<&str> = line.split_whitespace().collect();
        list1.push(words[0].parse::<i32>().unwrap());
        list2.push(words[1].parse::<i32>().unwrap());
    }

    list1.sort();
    list2.sort();

    for (index, _) in list1.iter().enumerate() {
        distance += (list1[index] - list2[index]).abs()
    }

    println!("{distance}")
}
