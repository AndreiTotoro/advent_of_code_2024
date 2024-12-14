use std::collections::{HashMap, HashSet};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coords {
    i: i32,
    j: i32,
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

fn move_in_direction(direction: &Direction) -> (i32, i32) {
    match direction {
        Direction::Up => (-1, 0),
        Direction::Down => (1, 0),
        Direction::Left => (0, -1),
        Direction::Right => (0, 1),
    }
}

fn simulate_guard_path(
    map: &Vec<Vec<char>>,
    obstruction: Option<(usize, usize)>,
) -> Option<HashSet<(i32, i32)>> {
    let mut working_map = map.clone();

    if let Some((oi, oj)) = obstruction {
        if working_map[oi][oj] != '.' {
            return None;
        }
        working_map[oi][oj] = '#';
    }

    let start_coord = find_start_coord(&working_map);
    let col_size = working_map.len() as i32;
    let row_size = working_map[0].len() as i32;

    let mut visited_positions = HashSet::new();
    visited_positions.insert((start_coord.i, start_coord.j));

    let mut path_coords = HashSet::new();
    path_coords.insert((start_coord.i, start_coord.j));

    let mut current_coords = start_coord;
    let mut direction = Direction::Up;

    let mut seen_states = HashMap::new();
    let mut move_count = 0;

    while !(current_coords.i == 0
        || current_coords.j == 0
        || current_coords.i == col_size - 1
        || current_coords.j == row_size - 1)
    {
        let state = (current_coords.i, current_coords.j, direction);

        // Check for loop
        if let Some(prev_move_count) = seen_states.insert(state, move_count) {
            return Some(path_coords);
        }

        let (di, dj) = move_in_direction(&direction);
        let (mi, mj) = (
            (current_coords.i + di) as usize,
            (current_coords.j + dj) as usize,
        );

        if working_map[mi][mj] == '#' {
            direction.change_direction();
        } else {
            current_coords.i = mi as i32;
            current_coords.j = mj as i32;

            if !path_coords.contains(&(current_coords.i, current_coords.j)) {
                path_coords.insert((current_coords.i, current_coords.j));
            }
        }

        move_count += 1;
    }

    None
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

fn main() {
    let file = File::open("data.txt").unwrap();
    let reader = BufReader::new(file);

    let mut map: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        map.push(line.chars().collect());
    }

    let start_coord = find_start_coord(&map);

    let mut loop_positions = 0;

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if (i as i32, j as i32) == (start_coord.i, start_coord.j) || map[i][j] != '.' {
                continue;
            }

            if let Some(_) = simulate_guard_path(&map, Some((i, j))) {
                loop_positions += 1;
            }
        }
    }

    println!("{}", loop_positions);
}
