/*
 * FILE: Day 10/Rust/sol.rs
 * Author: Alex Jones
 * Desc: Solution to day 10 problems (19 & 20) for Advent of Code 2024, solved in Rust.
 */
use itertools::Itertools;
use std::collections::HashSet;
use std::fs;

const DIRS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn trails_from(data: &Vec<Vec<u32>>, y: i32, x: i32) -> HashSet<Vec<(i32, i32)>> {
    if data[y as usize][x as usize] == 9 {
        return HashSet::from([vec![(y, x)]]);
    }
    let mut trailhead_ends = HashSet::new();
    for (dy, dx) in DIRS {
        let (ny, nx) = (y + dy, x + dx);
        if 0 > ny || ny >= data.len() as i32 || 0 > nx || nx >= data[0].len() as i32 {
            continue;
        }
        if data[ny as usize][nx as usize] == data[y as usize][x as usize] + 1 {
            let paths = trails_from(data, ny, nx);
            for mut path in paths {
                path.push((y, x));
                trailhead_ends.insert(path);
            }
        }
    }
    trailhead_ends
}

pub fn solve() {
    let contents: String = fs::read_to_string("Day 10/data.txt")
        .expect("Could not read data file")
        .trim()
        .replace("\r", "");
    let data: Vec<Vec<u32>> = contents
        .split("\n")
        .map(|row| row.chars().map(|char| char.to_digit(10).unwrap()).collect())
        .collect();

    let mut trails = Vec::new();
    for y in 0..data.len() {
        for x in 0..data[y].len() {
            if data[y][x] == 0 {
                trails.push(trails_from(&data, y as i32, x as i32));
            }
        }
    }

    println!(
        "Problem 19: {}",
        trails
            .iter()
            .map(|trailhead| trailhead.iter().map(|path| path[0]).unique().count())
            .sum::<usize>()
    );

    println!(
        "Problem 20: {}",
        trails
            .iter()
            .map(|trailhead| trailhead.len())
            .sum::<usize>()
    );
}
