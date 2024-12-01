/*
 * FILE: Day 01/Rust/sol.rs
 * Author: Alex Jones
 * Desc: Solution to day 1 problems (1 & 2) for Advent of Code 2024, solved in Rust.
 */
use std::fs;
use std::collections::HashMap;

use itertools::Itertools;

pub fn solve() {
    let contents: String = fs::read_to_string("Day 01/data.txt")
        .expect("Could not read data file")
        .replace("\r", "");
    let data: Vec<[u32; 2]> = contents
        .lines()
        .map(|row| {
            let items: Vec<u32> = row
                .split_whitespace()
                .map(|val| {val
                    .parse::<u32>()
                    .unwrap()
                })
                .collect();
            [items[0], items[1]]
        })
        .collect();

    let lists: [Vec<u32>; 2] = [
        data.iter().map(|x| x[0]).sorted().collect(),
        data.iter().map(|x| x[1]).sorted().collect(),
    ];
        
    println!("Problem 1: {}", lists[0].iter()
        .zip(lists[1].iter())
        .map(|(i, j)| {i.abs_diff(*j) as u64})
        .sum::<u64>());
    
    let mut counts: HashMap<u32, u32> = HashMap::new();
    for item in lists[1].clone() {
        *counts.entry(item).or_default() += 1;
    }
    
    println!("Problem 2: {}", lists[0].iter()
        .map(|i| {(i * counts.get(i).unwrap_or(&0)) as u64})
        .sum::<u64>());
}