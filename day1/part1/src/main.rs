use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut distance = 0;

    let file = File::open("data.txt")?;

    let reader = BufReader::new(file);

    let (mut list1, mut list2): (Vec<i32>, Vec<i32>) = reader
        .lines()
        .map(|line| {
            let line = line?;
            let words: Vec<&str> = line.split_whitespace().collect();

            let num1: i32 = words[0].parse()?;
            let num2: i32 = words[1].parse()?;

            Ok((num1, num2))
        })
        .collect::<Result<Vec<_>, Box<dyn Error>>>()?
        .into_iter()
        .unzip();

    list1.sort();
    list2.sort();

    for (index, _) in list1.iter().enumerate() {
        distance += (list1[index] - list2[index]).abs()
    }

    println!("{distance}");
    Ok(())
}
