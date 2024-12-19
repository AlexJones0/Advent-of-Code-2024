/*
 * FILE: Day 19/Rust/sol.rs
 * Author: Alex Jones
 * Desc: Solution to day 19 problems (37 & 38) for Advent of Code 2024, solved in Rust.
 */
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs;

type Memo = HashMap<String, u64>;
fn ways(memo: &mut Memo, patterns: &Vec<&str>, design: &str) -> u64 {
    let entry = memo.entry(design.to_string());
    if let Entry::Occupied(e) = entry {
        *e.get()
    } else if design.is_empty() {
        *entry.or_insert(1u64)
    } else {
        let result = patterns
            .iter()
            .filter(|&pattern| design.starts_with(pattern))
            .map(|&pattern| ways(memo, patterns, &design[pattern.len()..]))
            .sum();
        memo.insert(design.to_string(), result);
        result
    }
}

pub fn solve() {
    let contents: String = fs::read_to_string("Day 19/data.txt")
        .expect("Could not read data file")
        .trim()
        .replace("\r", "");
    let data: Vec<&str> = contents.split("\n\n").collect();

    let patterns: Vec<&str> = data[0].split(",").map(|pattern| pattern.trim()).collect();
    let designs: Vec<&str> = data[1].split("\n").collect();

    let mut memoized = Memo::new();
    let results: Vec<u64> = designs
        .iter()
        .map(|design| ways(&mut memoized, &patterns, design))
        .collect();
    println!("Problem 37: {}", results.iter().filter(|&&v| v > 0).count());
    println!("Problem 38: {}", results.iter().sum::<u64>());
}
