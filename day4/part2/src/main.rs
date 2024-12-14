use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn can_move_down(i: usize, col_size: usize) -> bool {
    col_size - 1 != i
}

fn can_move_up(i: usize) -> bool {
    i >= 1
}

fn can_move_left(j: usize) -> bool {
    j >= 1
}

fn can_move_right(j: usize, row_size: usize) -> bool {
    row_size - 1 != j
}

fn is_square(i: usize, j: usize, row_size: usize, col_size: usize) -> bool {
    can_move_down(i, col_size) && can_move_up(i) && can_move_left(j) && can_move_right(j, row_size)
}

fn main() {
    let mut occurances = 0;
    let file = File::open("data.txt").unwrap();
    let reader = BufReader::new(file);
    let mut mat: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        mat.push(line.chars().collect());
    }

    let col_size = mat.len();
    let row_size = mat[0].len();

    for (i, line) in mat.iter().enumerate() {
        for (j, &col) in line.iter().enumerate() {
            if col == 'A' {
                if is_square(i, j, row_size, col_size) {
                    if ((mat[i - 1][j - 1] == 'M' && mat[i + 1][j + 1] == 'S')
                        || (mat[i - 1][j - 1] == 'S' && mat[i + 1][j + 1] == 'M'))
                        && ((mat[i - 1][j + 1] == 'M' && mat[i + 1][j - 1] == 'S')
                            || (mat[i - 1][j + 1] == 'S' && mat[i + 1][j - 1] == 'M'))
                    {
                        occurances += 1;
                    }
                }
            }
        }
    }

    println!("{occurances}")
}
