/*
 * FILE: Day 02/Rust/sol.rs
 * Author: Alex Jones
 * Desc: Solution to day 2 problems (3 & 4) for Advent of Code 2024, solved in Rust.
 */
use std::fs;

pub fn is_safe(row: &[i32]) -> bool {
    let diffs = row.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();
    diffs.iter().all(|v| 1 <= *v && *v <= 3) || diffs.iter().all(|v| -3 <= *v && *v <= -1)
}

pub fn solve() {
    let contents: String = fs::read_to_string("Day 02/data.txt")
        .expect("Could not read data file")
        .trim()
        .replace("\r", "");
    let data: Vec<Vec<i32>> = contents
        .split("\n")
        .map(|row| {
            row.split_whitespace()
                .map(|value| value.parse().unwrap())
                .collect()
        })
        .collect();

    println!(
        "Problem 3: {}",
        data.iter().filter(|row| is_safe(row)).count()
    );

    println!(
        "Problem 4: {}",
        data.iter()
            .filter(|row| {
                (0..row.len()).any(|i| {
                    let mut removed = row.to_owned().clone();
                    removed.remove(i);
                    is_safe(&removed)
                })
            })
            .count()
    );
}
