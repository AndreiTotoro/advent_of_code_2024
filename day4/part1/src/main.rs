use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn can_move_down(i: usize, col_size: usize) -> bool {
    col_size - i >= 4
}

fn can_move_up(i: usize) -> bool {
    i >= 3
}

fn can_move_left(j: usize) -> bool {
    j >= 3
}

fn can_move_right(j: usize, row_size: usize) -> bool {
    row_size - j >= 4
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
            if col == 'X' {
                if can_move_down(i, col_size) {
                    if mat[i + 1][j] == 'M' && mat[i + 2][j] == 'A' && mat[i + 3][j] == 'S' {
                        occurances += 1;
                    }
                }
                if can_move_up(i) {
                    if mat[i - 1][j] == 'M' && mat[i - 2][j] == 'A' && mat[i - 3][j] == 'S' {
                        occurances += 1;
                    }
                }
                if can_move_left(j) {
                    if mat[i][j - 1] == 'M' && mat[i][j - 2] == 'A' && mat[i][j - 3] == 'S' {
                        occurances += 1;
                    }
                }
                if can_move_right(j, row_size) {
                    if mat[i][j + 1] == 'M' && mat[i][j + 2] == 'A' && mat[i][j + 3] == 'S' {
                        occurances += 1;
                    }
                }
                if can_move_left(j) && can_move_up(i) {
                    if mat[i - 1][j - 1] == 'M'
                        && mat[i - 2][j - 2] == 'A'
                        && mat[i - 3][j - 3] == 'S'
                    {
                        occurances += 1;
                    }
                }
                if can_move_right(j, row_size) && can_move_up(i) {
                    if mat[i - 1][j + 1] == 'M'
                        && mat[i - 2][j + 2] == 'A'
                        && mat[i - 3][j + 3] == 'S'
                    {
                        occurances += 1;
                    }
                }
                if can_move_left(j) && can_move_down(i, col_size) {
                    if mat[i + 1][j - 1] == 'M'
                        && mat[i + 2][j - 2] == 'A'
                        && mat[i + 3][j - 3] == 'S'
                    {
                        occurances += 1;
                    }
                }
                if can_move_right(j, row_size) && can_move_down(i, col_size) {
                    if mat[i + 1][j + 1] == 'M'
                        && mat[i + 2][j + 2] == 'A'
                        && mat[i + 3][j + 3] == 'S'
                    {
                        occurances += 1;
                    }
                }
            }
        }
    }

    println!("{occurances}")
}
