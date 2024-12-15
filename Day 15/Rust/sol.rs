/*
 * FILE: Day 15/Rust/sol.rs
 * Author: Alex Jones
 * Desc: Solution to day 15 problems (29 & 30) for Advent of Code 2024, solved in Rust.
 */
use itertools::Itertools;
use std::fs;

fn move_to_dir(move_char: u8) -> Option<(i32, i32)> {
    match move_char {
        b'<' => Some((0, -1)),
        b'^' => Some((-1, 0)),
        b'>' => Some((0, 1)),
        b'v' => Some((1, 0)),
        _ => None,
    }
}

fn widened(pos_char: u8) -> [u8; 2] {
    match pos_char {
        b'@' => [b'@', b'.'],
        b'O' => [b'[', b']'],
        c => [c, c],
    }
}

fn can_move(grid: &Vec<Vec<u8>>, pos: (i32, i32), dir: (i32, i32)) -> bool {
    let (ny, nx) = (pos.0 + dir.0, pos.1 + dir.1);
    let char = grid[ny as usize][nx as usize];
    if char == b'[' || char == b']' {
        let left = if char == b'[' { (ny, nx) } else { (ny, nx - 1) };
        let right = if char == b']' { (ny, nx) } else { (ny, nx + 1) };
        (dir == (0, 1) || can_move(grid, left, dir))
            && (dir == (0, -1) || can_move(grid, right, dir))
    } else if char == b'O' {
        can_move(grid, (ny, nx), dir)
    } else {
        char != b'#'
    }
}

fn make_move(grid: &mut Vec<Vec<u8>>, pos: (i32, i32), dir: (i32, i32), partial: bool) {
    let (ny, nx) = (pos.0 + dir.0, pos.1 + dir.1);
    let char = grid[pos.0 as usize][pos.1 as usize];
    if !partial && dir.0 != 0 {
        if char == b'[' {
            make_move(grid, (pos.0, pos.1 + 1), dir, true);
        } else if char == b']' {
            make_move(grid, (pos.0, pos.1 - 1), dir, true);
        }
    }
    if grid[ny as usize][nx as usize] != b'.' {
        make_move(grid, (ny, nx), dir, false);
    }
    grid[ny as usize][nx as usize] = char;
    grid[pos.0 as usize][pos.1 as usize] = b'.';
}

fn simulate(grid: &mut Vec<Vec<u8>>, moves: &str) {
    let mut robot = None;
    for (i, row) in grid.iter().enumerate() {
        for (j, &char) in row.iter().enumerate() {
            if char == b'@' {
                robot = Some((i as i32, j as i32));
                break;
            }
        }
    }
    let mut robot = robot.unwrap();
    for move_char in moves.bytes() {
        let dir = move_to_dir(move_char).unwrap();
        if can_move(grid, robot, dir) {
            make_move(grid, robot, dir, false);
            robot = (robot.0 + dir.0, robot.1 + dir.1);
        }
    }
}

fn get_total_gps(grid: &[Vec<u8>]) -> u32 {
    let mut total = 0u32;
    for (i, row) in grid.iter().enumerate() {
        for (j, &char) in row.iter().enumerate() {
            if char == b'O' || char == b'[' {
                total += 100 * (i as u32) + (j as u32);
            }
        }
    }
    total
}

pub fn solve() {
    let contents: String = fs::read_to_string("Day 15/data.txt")
        .expect("Could not read data file")
        .trim()
        .replace("\r", "");
    let data: (&str, &str) = contents.split("\n\n").collect_tuple().unwrap();

    let grid: Vec<Vec<u8>> = data.0.split("\n").map(|s| s.bytes().collect()).collect();
    let moves = data.1.replace('\n', "");

    let mut p1 = grid.clone();
    simulate(&mut p1, &moves);
    println!("Problem 29: {}", get_total_gps(&p1));

    let mut p2: Vec<Vec<u8>> = grid
        .iter()
        .map(|row| row.iter().flat_map(|&c| widened(c)).collect())
        .collect();
    simulate(&mut p2, &moves);
    println!("Problem 30: {}", get_total_gps(&p2));
}
