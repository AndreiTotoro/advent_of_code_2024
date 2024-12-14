use std::io::Write;
use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader},
};

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Coords {
    i: i32,
    j: i32,
}

fn write_map_to_file(map: &Vec<Vec<char>>, filename: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(filename)
        .unwrap();

    for row in map {
        writeln!(file, "{}", row.iter().collect::<String>()).unwrap();
    }
    writeln!(file).unwrap();
}

impl Direction {
    fn change_direction(&mut self) {
        *self = match *self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

fn find_start_coord(map: &Vec<Vec<char>>) -> Coords {
    for (i, row) in map.iter().enumerate() {
        for (j, &col) in row.iter().enumerate() {
            if col == '^' {
                return Coords {
                    i: i as i32,
                    j: j as i32,
                };
            }
        }
    }
    panic!("Coords not found!")
}

fn check_for_victory(current_coords: &Coords, row_size: i32, col_size: i32) -> bool {
    current_coords.i == 0
        || current_coords.j == 0
        || current_coords.i == col_size - 1
        || current_coords.j == row_size - 1
}

fn move_in_direction(direction: &Direction) -> (i32, i32) {
    match direction {
        Direction::Up => (-1, 0),
        Direction::Down => (1, 0),
        Direction::Left => (0, -1),
        Direction::Right => (0, 1),
    }
}

fn move_guard(
    current_coords: &mut Coords,
    direction: &mut Direction,
    unique_moves: &mut i32,
    map: &mut Vec<Vec<char>>,
) {
    let (di, dj) = move_in_direction(&direction);
    let (mi, mj) = (
        (current_coords.i + di) as usize,
        (current_coords.j + dj) as usize,
    );

    if map[mi][mj] == '#' {
        direction.change_direction();
    } else if map[mi][mj] == '.' {
        current_coords.i = mi as i32;
        current_coords.j = mj as i32;
        map[mi][mj] = 'X';
        *unique_moves += 1;
    } else if map[mi][mj] == 'X' {
        current_coords.i = mi as i32;
        current_coords.j = mj as i32;
    }
}

fn main() {
    let file = File::open("data.txt").unwrap();
    let reader = BufReader::new(file);

    let mut map: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        map.push(line.chars().collect());
    }

    let start_coord = find_start_coord(&map);
    let start_direction = Direction::Up;
    let col_size = map.len() as i32;
    let row_size = map[0].len() as i32;

    let mut unique_moves = 1 as i32;
    let mut direction = start_direction;
    let mut current_coords = start_coord;

    while !check_for_victory(&current_coords, col_size, row_size) {
        move_guard(
            &mut current_coords,
            &mut direction,
            &mut unique_moves,
            &mut map,
        )
    }

    write_map_to_file(&map, "completion_map.txt");
    println!("{unique_moves}")
}
