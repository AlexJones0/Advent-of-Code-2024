/*
 * FILE: Day 07/Rust/sol.rs
 * Author: Alex Jones
 * Desc: Solution to day 7 problems (13 & 14) for Advent of Code 2024, solved in Rust.
 */
use std::fs;

fn satisfies(
    target: i64,
    current: i64,
    vals: &Vec<i64>,
    start: usize,
    operators: &Vec<fn(i64, i64) -> i64>,
) -> bool {
    if current > target || start >= vals.len() {
        return current == target;
    }
    operators
        .iter()
        .map(|op| satisfies(target, op(current, vals[start]), vals, start + 1, operators))
        .any(|x| x)
}

fn total_valid(data: &[(i64, Vec<i64>)], operators: &Vec<fn(i64, i64) -> i64>) -> i64 {
    data.iter()
        .filter(|(target, vals)| satisfies(*target, vals[0], vals, 1, operators))
        .map(|x| x.0)
        .sum()
}

pub fn solve() {
    let contents: String = fs::read_to_string("Day 07/data.txt")
        .expect("Could not read data file")
        .trim()
        .replace("\r", "");
    let data: Vec<(i64, Vec<i64>)> = contents
        .split("\n")
        .map(|line| {
            let mut parts = line.split(": ");
            (
                parts.next().unwrap().parse::<i64>().unwrap(),
                parts
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .map(|val| val.parse::<i64>().unwrap())
                    .collect(),
            )
        })
        .collect();

    let mut operators: Vec<fn(i64, i64) -> i64> = vec![|x, y| x + y, |x, y| x * y];
    println!("Problem 13: {}", total_valid(&data, &operators));
    operators.push(|x, y| (x.to_string() + &y.to_string()).parse::<i64>().unwrap());
    println!("Problem 14: {}", total_valid(&data, &operators));
}
