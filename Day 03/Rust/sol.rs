/*
 * FILE: Day 03/Rust/sol.rs
 * Author: Alex Jones
 * Desc: Solution to day 3 problems (5 & 6) for Advent of Code 2024, solved in Rust.
 */
use lazy_static::lazy_static;
use regex::Regex;
use std::fs;

lazy_static! {
    static ref NUM_REGEX: Regex = Regex::new(r"[0-9]{1,3}").unwrap();
    static ref STMT_REGEX: Regex =
        Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)|do\(\)|don't\(\)").unwrap();
}

fn calc(muls: &Vec<&&str>) -> u64 {
    muls.iter()
        .map(|stmt| {
            let matches: Vec<u32> = NUM_REGEX
                .captures_iter(stmt)
                .map(|c| c.extract::<0>().0.parse::<u32>().unwrap())
                .collect();
            (matches[0] * matches[1]) as u64
        })
        .sum()
}

pub fn solve() {
    let contents: String = fs::read_to_string("Day 03/data.txt")
        .expect("Could not read data file")
        .trim()
        .replace("\r", "");

    let stmts: Vec<&str> = STMT_REGEX
        .captures_iter(contents.as_str())
        .map(|c| c.extract::<0>().0)
        .collect();
    let muls = stmts.iter().filter(|stmt| stmt.starts_with('m')).collect();
    println!("Problem 5: {}", calc(&muls));

    let mut state = true;
    let filtered = stmts
        .iter()
        .filter(|stmt| {
            if **stmt == "do()" || **stmt == "don't()" {
                state = **stmt == "do()";
                false
            } else {
                state
            }
        })
        .collect();
    println!("Problem 6: {}", calc(&filtered));
}
